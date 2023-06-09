terraform {
    required_version = ">=0.14"
    
    required_providers {
        google = {
            source = "hashicorp/google"
            version = "4.65.2"
        }

        google-beta = {
          version = ">= 3.8"
        }
    }
}

provider "google" {
    credentials = file("creds.json")
    project = var.project_id
    region = var.region
    zone = var.zone
}

module "project-services" {
    source                      = "terraform-google-modules/project-factory/google//modules/project_services"
    version                     = "13.0.0"
    disable_services_on_destroy = false

    project_id  = var.project_id
    enable_apis = var.enable_apis

    activate_apis = [
        "compute.googleapis.com",
        "cloudapis.googleapis.com",
        "vpcaccess.googleapis.com",
        "servicenetworking.googleapis.com",
        "cloudbuild.googleapis.com",
        "sql-component.googleapis.com",
        "sqladmin.googleapis.com",
        "storage.googleapis.com",
        "run.googleapis.com",
        "cloudresourcemanager.googleapis.com",
        "iam.googleapis.com",
        "secretmanager.googleapis.com",
    ]
}

data "google_project" "project" {
    project_id = var.project_id
}

##################
## SERVICE ACCOUNT
##################

resource "google_service_account" "runsa" {
    project      = var.project_id
    account_id   = "${var.project_name}-run-sa"
    display_name = "Service Account for Cloud Run"
}

resource "google_project_iam_member" "allrun" {
    for_each = toset(var.run_roles_list)
    project  = data.google_project.project.number
    role     = each.key
    member   = "serviceAccount:${google_service_account.runsa.email}"
}

##########
## NETWORK
##########

resource "google_compute_network" "main" {
    name                    = "${var.project_name}-private-network"
    auto_create_subnetworks = true
    project                 = var.project_id
}

resource "google_compute_global_address" "main" {
  name          = "${var.project_name}-vpc-address"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = google_compute_network.main.name
  project       = var.project_id
}

resource "google_service_networking_connection" "main" {
  network                 = google_compute_network.main.self_link
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.main.name]
}

resource "google_vpc_access_connector" "main" {
  project        = var.project_id
  name           = "${var.project_name}-cx"
  ip_cidr_range  = "10.10.0.0/28"
  network        = google_compute_network.main.name
  region         = var.region
  max_throughput = 300
}

###########
## DATABASE
###########

resource "random_id" "id" {
  byte_length = 2
}

# Handle Database
resource "google_sql_database_instance" "main" {
  name             = "${var.project_name}-db-${random_id.id.hex}"
  database_version = "POSTGRES_14"
  region           = var.region
  project          = var.project_id

  settings {
    tier                  = "db-f1-micro"
    disk_autoresize       = true
    disk_autoresize_limit = 0
    disk_size             = 10
    disk_type             = "PD_SSD"
    user_labels           = var.labels
    ip_configuration {
      ipv4_enabled    = false
      private_network = "projects/${var.project_id}/global/networks/${google_compute_network.main.name}"
    }
    location_preference {
      zone = var.zone
    }
    database_flags {
      name  = "cloudsql.iam_authentication"
      value = "on"
    }
  }
  deletion_protection = false

  depends_on = [
    google_service_networking_connection.main
  ]
}

resource "google_sql_user" "main" {
  project         = var.project_id
  name            = "${google_service_account.runsa.account_id}@${var.project_id}.iam"
  type            = "CLOUD_IAM_SERVICE_ACCOUNT"
  instance        = google_sql_database_instance.main.name
  deletion_policy = "ABANDON"
}

resource "google_sql_database" "database" {
  project         = var.project_id
  name            = "todo"
  instance        = google_sql_database_instance.main.name
  deletion_policy = "ABANDON"
}

############
## CLOUD RUN
############

resource "google_cloud_run_v2_service" "default" {
  name     = "cloudrun-service"
  location = "us-central1"
  ingress = "INGRESS_TRAFFIC_ALL"

  template {
    scaling {
      max_instance_count = 2
    }

    volumes {
      name = "cloudsql"
      cloud_sql_instance {
        instances = [google_sql_database_instance.main.connection_name]
      }
    }

    containers {
      image = "us-docker.pkg.dev/cloudrun/container/hello"

      env {
        name = "FOO"
        value = "bar"
      }
      
#       env {
#         name = "SECRET_ENV_VAR"
#         value_source {
#           secret_key_ref {
#  #           secret = google_secret_manager_secret.secret.secret_id
#             version = "1"
#           }
#         }
#       }
      
      volume_mounts {
        name = "cloudsql"
        mount_path = "/cloudsql"
      }
    }
  }

  traffic {
    type = "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    percent = 100
  }
  #depends_on = [google_secret_manager_secret_version.secret-version-data]
}

# resource "google_secret_manager_secret" "secret" {
#   secret_id = "secret-1"
#   replication {
#     automatic = true
#   }
# }

# resource "google_secret_manager_secret_version" "secret-version-data" {
#   secret = google_secret_manager_secret.secret.name
#   secret_data = "secret-data"
# }

# resource "google_secret_manager_secret_iam_member" "secret-access" {
#   secret_id = google_secret_manager_secret.secret.id
#   role      = "roles/secretmanager.secretAccessor"
#   member    = "serviceAccount:${data.google_project.project.number}-compute@developer.gserviceaccount.com"
#   depends_on = [google_secret_manager_secret.secret]
# }
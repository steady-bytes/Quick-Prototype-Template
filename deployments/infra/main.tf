terraform {
    required_providers {
        google = {
            source = "hashicorp/google"
            version = "4.65.2"
        }
    }
}

provider "google" {
  project     = ""
  region      = "us-central1"
}
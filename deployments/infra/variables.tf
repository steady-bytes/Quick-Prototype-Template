############
## VARIABLES
############

variable "project_name" {
  type        = string
  description = "The name of this particular deployment."
  default     = "template"
}

variable "project_id" {
    type = string 
    description = "The ID that will be used for this project"
    default = "template"
}

variable "org_id" {
    type = string
    description = "Organization ID"
}

variable "billing_account" {
    type = string
    description = "The billing account to be used for the projects deployment"
}

variable "region" {
  type        = string
  description = "The Compute Region to deploy to"
  default = "us-central1"
}

variable "zone" {
  type        = string
  description = "The Compute Zone to deploy to"
  default = "us-central1-c"
}

variable "enable_apis" {
    type = string 
    description = "Enable google api's so terraform can deploy the resources it needs"
    default = true
}

variable "labels" {
  type        = map(string)
  description = "A map of labels to apply to contained resources."
  default     = { "rapid-prototype-deployment" = true }
}

variable "run_roles_list" {
  description = "The list of roles that run needs"
  type        = list(string)
  default = [
    "roles/cloudsql.instanceUser",
    "roles/cloudsql.client",
  ]
}
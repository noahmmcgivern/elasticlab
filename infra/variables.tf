variable "ACCESS_KEY" {}
variable "SECRET_KEY" {}

provider "aws" {
    region     = "us-east-1"
    access_key = var.ACCESS_KEY
    secret_key = var.SECRET_KEY
}
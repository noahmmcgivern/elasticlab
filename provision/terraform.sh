#!/bin/bash

# Install unzip
apt-get install unzip 

# Download Terraform
curl -s -L -o "terraform.zip" "https://releases.hashicorp.com/terraform/0.13.4/terraform_0.13.4_linux_amd64.zip"

# Extract Terraform
unzip "terraform.zip" && rm "terraform.zip"

# Install Terraform
mv "terraform" "/usr/local/bin/"
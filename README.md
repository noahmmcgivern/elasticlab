# ElasticLab
CLI to control [AWS Data Analytics infrastructure](https://aws.amazon.com/emr/) via [Terraform](https://www.terraform.io/)
---
## Requirements
- [Docker](https://www.docker.com/)
- [Vagrant (only for development)](https://www.vagrantup.com/)

Also:
- [AWS Access Key / Secret Key](https://console.aws.amazon.com/iam/home?#/security_credentials)
## Getting started
Build and run the Docker image: (go get a coffee when building for the first time...)
```
make
```
If successful, you will have a root shell.
## ElasticLab CLI (el) Tutorial
Get help:
```
el
```
Set Access Key / Secret Key:
```
el key
```
Get available infrastructure:
```
el set
```
Set number of buckets to 2:
```
el set s3 2
```
Get planned infrastructure from lockfile:
```
el get
```
Get staged infrastructure as list of Terraform files:
```
el get stage
```
Set up the buckets:
```
el go
```
Scale up buckets:
```
el set s3 6
el go
```
Get live infrastructure:
```
el get aws
```
Set up a cluster:
```
el set emr 1
el go
```
Scale down:
```
el set emr 0
el set s3 1
el go
```
Destroy infrastructure when finished:
```
el des
```

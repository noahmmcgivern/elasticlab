resource "aws_s3_bucket" "BUCKET" {
  bucket = "BUCKET"

  tags = {
    Name = "elasticlab"
  }
}
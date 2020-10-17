resource "aws_s3_bucket" "NAME" {
  bucket = "NAME"

  tags = {
    Name = "elasticlab"
  }
}

resource "aws_s3_bucket_public_access_block" "NAME" {
  bucket = aws_s3_bucket.NAME.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}
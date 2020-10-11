resource "aws_dynamodb_table" "NAME" {
  name           = "NAME"
  billing_mode   = "PROVISIONED"
  read_capacity  = 3
  write_capacity = 3
  hash_key       = "id"

  attribute {
    name = "id"
    type = "S"
  }

  tags = {
    Name = "elasticlab"
  }
}
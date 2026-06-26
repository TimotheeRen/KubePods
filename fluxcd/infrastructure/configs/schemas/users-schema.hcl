table "users" {
  schema = schema.public
  column "username" {
    type = text
  }
  column "email" {
    type = text
  }
  column "password" {
    type = text
  }
  column "utilization" {
    type = integer
  }
  primary_key {
    columns = [
      column.username
    ]
  }
}

schema "public" {}
schema "private" {}

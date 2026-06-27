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
    default = 0
  }
  column "desktops_utilization" {
    type = jsonb
    null = true
  }
  primary_key {
    columns = [
      column.username
    ]
  }
}

schema "public" {}
schema "private" {}

table "desktops" {
  schema = schema.public
  column "id" {
    type = text
  }
  column "name" {
    type = text
  }
  column "username" {
    type = text
  }
  column "distribution" {
    type = text
  }
  column "desktop_environment" {
    type = text
  }
  primary_key {
    columns = [
      column.id
    ]
  }
}

schema "public" {}
schema "private" {}

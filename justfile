prepare:
  #!/usr/bin/env bash

  database=$(mktemp)
  cargo sqlx prepare --database-url "sqlite:$database"
  rm $database

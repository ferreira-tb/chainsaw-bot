```bash
cargo install manatsu sea-orm-cli
cargo install tauri-cli --version "^2.0.0-beta"
manatsu create

cd PROJECT_NAME
pnpm run dev
```

## Database

### Migration

```sh
# Generate a new migration file.
sea-orm-cli migrate generate MIGRATION_NAME

# Apply all pending migrations.
sea-orm-cli migrate up

# Rollback last applied migration
sea-orm-cli migrate down

# Drop all tables from the database, then reapply all migrations.
sea-orm-cli migrate fresh

# Rollback all applied migrations, then reapply all migrations.
sea-orm-cli migrate refresh

# Rollback all applied migrations.
sea-orm-cli migrate reset
```

### Generating Entities

[`DATABASE_URL`](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/#configure-environment) must be set as an environment variable.

```sh
sea-orm-cli generate entity -o src-tauri/src/database/entities --with-serde both --model-extra-attributes "serde(rename_all(serialize = \"camelCase\"))" --model-extra-attributes "serde(rename_all(deserialize = \"snake_case\"))" --serde-skip-deserializing-primary-key
```

## License

[MIT](https://raw.githubusercontent.com/ferreira-tb/manatsu/main/LICENSE)

Copyright (c) 2024 Andrew Ferreira

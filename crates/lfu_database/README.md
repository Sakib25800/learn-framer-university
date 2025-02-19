# lfu_database

This package contains the database schema as derived by `diesel print-schema` from the databse after all migrations have been applied.

After creating new migrations (via `diesel migration generate`), update the schema by executing:
```console
diesel print-schema > crates/lfu_database/src/schema.rs
```

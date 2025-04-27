# framer_university_database

This package provides access to the framer-university database.

1. Generate a new migration
   ```console
   $ cargo sqlx migrate add <new-migration>
   ```
2. Change the migration manually in `migrations/<timestamp>-<new-migration>.sql`.
3. Apply migrations to the **Postgres** DB.
   ```console
   $ cargo sqlx migrate run
   ```

### Generate `.sqlx` directory

```console
$ cargo sqlx prepare -- --package framer_university_database --all-targets
```

You should also commit the changes to the `.sqlx` directory.

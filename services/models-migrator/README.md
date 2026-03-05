# MLHub Models API Migrator

This program will run all migrations in `src/migrations.rs`. This migrator is deployed as an init container of the Models API deployment. Each migration is tracked with an entry in the Artifacts DB created by the migration library `fiala_mongodb_migrator`.
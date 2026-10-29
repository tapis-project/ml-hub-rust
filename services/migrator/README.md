# MLHub Migrator

This service runs all MLHub MongoDB migrations in bounded-context groups. It preserves the
historical migration identifiers stored in the shared `migrations` collection and stops before
running later groups when a group fails.


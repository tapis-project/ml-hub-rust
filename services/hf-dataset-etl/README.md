# Hugging Face Dataset ETL Pipeline

This pipeline extracts open, public, non-gated Dataset repository snapshots from Hugging Face,
transforms their repository files into MLHub Dataset items, and registers them in the global MLHub
tenant.

The extractor resolves file metadata for each repository so every item has an exact byte size. The
transform/load stage calculates the Dataset size from those items, filters provider tags to MLHub
limits, and skips an `id` and commit `sha` already registered by the pipeline. A new commit SHA is
registered as a new Dataset snapshot.

Run the configured Kubernetes Job through the component manager with a valid Hugging Face token.
`MAX_RECORDS` limits successfully extracted records and `MAX_PROCESSABLE_ENTRIES` limits records
read by the transform/load stage; `-1` means unlimited.

# Model ETL Pipeline

Run the following from the root of the project: `./dev run hf-model-etl -a $HF_TOKEN` where `$HF_TOKEN` is a valid Huggingface Hub access token. This will run the 3 stages of the ETL pipeline.

The extractor reads Hugging Face's aggregate `usedStorage` model property from
the ordinary model metadata response. It does not request file metadata or add
`blobs=true`. Rate-limited metadata requests honor `Retry-After`, then Hugging
Face's `RateLimit` reset value, with bounded exponential backoff as a fallback.
Progress is flushed to stdout every `PROGRESS_INTERVAL` examined models
(default `100`), when a shard is written, and when extraction completes.

Step 1: Extract - This step pulls metadata from huggingface and saves batches of the metadata to jsonl files.
Step 2: Transform + Load - These next steps convert each entry to an ExternalModel and store it in the `EXTERNAL_MODELS` collection in MongoDB.

# Model Metadata ETL Pipeline

Run the following from the root of the project: `./dev run hf-model-etl -a $HF_TOKEN` where `$HF_TOKEN` is a valid Huggingface Hub access token. This will run the 3 stages of the ETL pipeline.

Step 1: Extract - This step pulls metadata from huggingface and saves batches of the metadata to jsonl files.
Step 2: Transform + Load - These next steps convert each entry to MLHub model metadata format and stores them in MODEL_METADATA collection in the MLHub mongodb.
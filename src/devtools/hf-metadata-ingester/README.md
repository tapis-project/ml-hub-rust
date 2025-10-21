**Model Metadata Ingestion Pipeline**

- Make a mapping from the tasks API in huggingface to Skills in the MLHub models api. https://huggingface.co/api/tasks. These task objects returned from teh api often have which libraries are compatible with the task type
    - Map huggingface metadata to mlhub metadata
    - hf -> mlh
    - tags -> keywords
        - license:<licence> -> license
- List and store huggingface models ordered by popularity
    - as many as possible. Probably 1000 at a time.
    - provide cursor in subsequent requests to get next pages
    - respect the retry after header
    - ensure the response contains the full metada including the config if there is one with config=true and full=true query params

- Convert the hugging face metadata to mlhub metadata.
    - Useful metadata from the hf metadata that doesn't conform to the mlhub metadata can be added to annotations
    -
    - Add annotations for the libraries that can be used to deploy a model. In this annotation, we
    can store the model.config
    ```json {
        "annotations": {
            "deployment_strategies": {
                "libraries": {
                    "transformers": {
                        "TextGeneration": {
                            "auto_map": {
                                "AutoConfig": "modeling_deepseekocr.DeepseekOCRConfig",
                                "AutoModel": "modeling_deepseekocr.DeepseekOCRForCausalLM",
                                "AutoModelForCausalLM": true
                            }
                        }
                    },
                    "diffusors": "...",
                    "gguf": 
                }
            },
        }
    }``` 

    This `auto_map` properties can be used to instantiate the correct classes for the transformers library

- When creating deploment specifications, prcess by task type/skill from simplest to complex. Ex. From NLP-related skills -> Vision skills -> Audio skills -> Multi Modal skills

IDEAS
    - Rank model metadata by completeness/utility. Fields that would grant a higher rank would include training properties/metrics, model IO diminsions and datatypes, hyperparameters etc.
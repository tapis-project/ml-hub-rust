# Model Metadata Ingestion Pipeline

0. Run the following from the root of the project: `./manage gen hf-metadata-ingester`

0. Make a mapping from the tasks API in huggingface to Skills in the MLHub models api. https://huggingface.co/api/tasks. These task objects returned from teh api often have which libraries are compatible with the task type

0. Map huggingface metadata to mlhub metadata
    ```json
    {
        "artifact_id": null,
        "name": "<mode.id>",
        "canonical": {
            "uri": "https://huggingface.co/<model.id>",
            "hash": "<model.sha>"
        },
        "license": ["<model.tags.*.license:<license>>"],
        "keywords": ["<model.tags.*>"],
        "annotations": {
            "huggingface": "<All relevant hf model metadata here>"
        },
        "skills": ["Skill::from('<model.pipeline_tag>')"],
        "inference_software_dependencies": [ "<model.library_name>" ]
    }
    ```
    
0. List and store huggingface models ordered by popularity
    - as many as possible. Probably 1000 at a time.
    - provide cursor in subsequent requests to get next pages
    - respect the retry after header
    - ensure the response contains the full metadata including the config (if there is one) with config=true and full=true query params

0. Convert the hugging face metadata to mlhub metadata.
    - Useful metadata from the hf metadata that doesn't conform to the mlhub metadata can be added to annotations

0. Define the **AutomatedDeploymentStrategy**
    - composed of **Vec<AutomatedDeploymentStrategyRule>**

0. Create a set of **AutomatedDeploymentStrategyRules**
    - model.license not in [<approved license list>]
    - model.

0. Use the **AutomatedDeploymentStrategyRules** to generate **AutomatedDeploymentStrategies**
    ```json
    {
        "platform": "oneOf(Platform) ex. Platform::TaccTapisPods, Platform::TaccTapisJobs",
        "as_rest_api": true/false,
        "container_runtime": Docker|Singularity/Apptainer|null
        "python": {
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
                "diffusers": {},
                "keras": {},
                "transformers.js": {},
                "scikit-learn": {},
                "pytorch": {}
            }
        }
    }
    ``` 

    This `auto_map` properties can be used to instantiate the correct classes for the transformers library

- When creating deploment specifications, prcess by task type/skill from simplest to complex. Ex. From NLP-related skills -> Vision skills -> Audio skills -> Multi Modal skills

IDEAS
    - Rank model metadata by completeness/utility. Fields that would grant a higher rank would include training properties/metrics, model IO diminsions and datatypes, hyperparameters etc.
use hf_hub::api::sync::Api;

pub struct HuggingFaceClient {}

impl HuggingFaceClient {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn call(&self) {
        let api = Api::new().unwrap();

        let repo = api.model("bert-base-uncased".to_string());
        let _filename = repo.get("config.json").unwrap();
        println!("Testing _filename: {}", _filename.display())
    }
}
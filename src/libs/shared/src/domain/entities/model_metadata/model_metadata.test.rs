#[cfg(test)]
mod model_metadata_test {
    use serde_json::Value;
    use crate::domain::entities::model_metadata::fixtures::full_model_metadata;
    
    #[test]
    fn test_get_field_value_at_field_path() {
        let model_metadata = full_model_metadata();
        let gated: Value = model_metadata
            .get_field_value_at_field_path(&vec!["annotations".into(), "canonical".into(), "gated".into()])
            .unwrap()
            .into();

        println!("GATED: {:#?}", gated);

        let private: Value = model_metadata
            .get_field_value_at_field_path(&vec!["annotations".into(), "canonical".into(), "private".into()])
            .unwrap()
            .into();

        assert!(gated.as_bool().unwrap() == false);
        assert!(private.as_bool().unwrap() == true);
    }
}
#[cfg(test)]
mod model_metadata_test {
    use serde_json::Value;
    use crate::domain::entities::model_metadata::fixtures::full_model_metadata;
    
    #[test]
    fn test_get_field_value_at_valid_field_path() {
        let model_metadata = full_model_metadata();
        let gated: Value = model_metadata
            .get_field_value_at_field_path(&vec!["canonical".into(), "gated".into()])
            .unwrap()
            .into();

        let private: Value = model_metadata
            .get_field_value_at_field_path(&vec!["canonical".into(), "private".into()])
            .unwrap()
            .into();

        assert!(gated.as_bool().unwrap() == false);
        assert!(private.as_bool().unwrap() == true);
    }

    #[test]
    fn test_get_field_value_at_invalid_field_path() {
        let model_metadata = full_model_metadata();
        let maybe_field_value = model_metadata
            .get_field_value_at_field_path(&vec!["nonexistent".into()]);

        assert!(maybe_field_value.is_err());
    }
}
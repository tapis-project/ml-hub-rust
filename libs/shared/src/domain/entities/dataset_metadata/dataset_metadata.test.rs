#[cfg(test)]
mod dataset_metadata_test {
    use serde_json::Value;
    use crate::domain::entities::dataset_metadata::fixtures::full_dataset_metadata;
    
    #[test]
    fn test_get_field_value_at_field_path() {
        let dataset_metadata = full_dataset_metadata();
        let gated: Value = dataset_metadata
            .get_field_value_at_field_path(&vec!["annotations".into(), "canonical".into(), "gated".into()])
            .unwrap()
            .into();

        let private: Value = dataset_metadata
            .get_field_value_at_field_path(&vec!["annotations".into(), "canonical".into(), "private".into()])
            .unwrap()
            .into();

        assert!(gated.as_bool().unwrap() == false);
        assert!(private.as_bool().unwrap() == true);
    }
}
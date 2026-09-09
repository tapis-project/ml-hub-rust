#[cfg(test)]
mod model_test {
    use crate::domain::entities::model::fixtures::full_model;
    use serde_json::Value;

    #[test]
    fn test_get_field_value_at_valid_field_path() {
        let model = full_model();
        let gated: Value = model
            .get_field_value_at_field_path(&vec!["canonical".into(), "gated".into()])
            .unwrap()
            .into();

        let private: Value = model
            .get_field_value_at_field_path(&vec!["canonical".into(), "private".into()])
            .unwrap()
            .into();

        assert!(gated.as_bool().unwrap() == false);
        assert!(private.as_bool().unwrap() == true);
    }

    #[test]
    fn test_get_field_value_at_invalid_field_path() {
        let model = full_model();
        let maybe_field_value = model.get_field_value_at_field_path(&vec!["nonexistent".into()]);

        assert!(maybe_field_value.is_err());
    }
}

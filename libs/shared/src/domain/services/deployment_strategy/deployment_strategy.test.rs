#[cfg(test)]
mod deployment_strategy_test {
    use super::super::evaluate_rule;
    use crate::domain::entities::deployment_strategy::rule_set::Rule;
    use crate::domain::entities::model::fixtures::full_model;
    use crate::domain::entities::operator::Operator;
    use serde_json::Value;

    #[test]
    fn test_in() {
        let name_in_value = Rule {
            field_path: vec!["name".into()],
            operator: Operator::In,
            value: Value::Array(vec!["foo".into(), "bar".into()]),
        };

        let model = full_model();

        assert!(model.name.clone() == String::from("foo"));
        assert!(evaluate_rule(&model, &name_in_value).unwrap());
    }

    #[test]
    fn test_not_in() {
        let name_not_in_value = Rule {
            field_path: vec!["name".into()],
            operator: Operator::NotIn,
            value: Value::Array(vec!["bin".into(), "baz".into()]),
        };

        let model = full_model();
        assert!(model.name.clone() == String::from("foo"));

        assert!(evaluate_rule(&model, &name_not_in_value).unwrap());

        let name_missing_from_value = Rule {
            field_path: vec!["name".into()],
            operator: Operator::In,
            value: Value::Array(vec!["bin".into(), "baz".into()]),
        };
        
        assert!(!evaluate_rule(&model, &name_missing_from_value).unwrap());
    }

    #[test]
    fn test_contains() {
        let libraries_contains_value = Rule {
            field_path: vec!["libraries".into()],
            operator: Operator::Contains,
            value: Value::String("transformers".into()),
        };

        let model = full_model();

        assert!(model.libraries.is_some());
        assert!(model.libraries.clone().unwrap().contains(&String::from("transformers")));
        assert!(evaluate_rule(&model, &libraries_contains_value).unwrap());
    }

    #[test]
    fn test_eq_neq() {
        let eq = Rule {
            field_path: vec!["name".into()],
            operator: Operator::Eq,
            value: Value::String("foo".into()),
        };

        let model = full_model();

        assert!(model.name.clone() == String::from("foo"));
        assert!(evaluate_rule(&model, &eq).unwrap());

        let neq = Rule {
            field_path: vec!["name".into()],
            operator: Operator::Neq,
            value: Value::String("bar".into()),
        };
        
        assert!(evaluate_rule(&model, &neq).unwrap());
    }

    // #[test]
    // fn test_gt_lt_gte_lte() {
    //     let model = full_model();

    //     let lt = Rule {
    //         field_path: vec!["inference_hardware".into(), "memory_gb".into()],
    //         operator: Operator::Lt,
    //         value: Value::Number(6.into()),
    //     };
    //     assert!(evaluate_rule(&model, &lt).unwrap());

    //     let gt = Rule {
    //         field_path: vec!["inference_hardware".into(), "memory_gb".into()],
    //         operator: Operator::Gt,
    //         value: Value::Number(4.into()),
    //     };
    //     assert!(evaluate_rule(&model, &gt).unwrap());

    //     let lte = Rule {
    //         field_path: vec!["inference_hardware".into(), "memory_gb".into()],
    //         operator: Operator::Lte,
    //         value: Value::Number(5.into()),
    //     };
    //     assert!(evaluate_rule(&model, &lte).unwrap());

    //     let gte = Rule {
    //         field_path: vec!["inference_hardware".into(), "memory_gb".into()],
    //         operator: Operator::Gte,
    //         value: Value::Number(5.into()),
    //     };
    //     assert!(evaluate_rule(&model, &gte).unwrap());
    // }

    #[test]
    fn test_all_in() {
        let libraries_all_in = Rule {
            field_path: vec!["libraries".into()],
            operator: Operator::AllIn,
            value: Value::Array(vec!["transformers".into(), "diffusers".into()]),
        };

        let model = full_model();
        assert!(model.libraries.is_some());
        assert!(model.libraries.clone().unwrap().contains(&String::from("transformers")));
        assert!(model.libraries.clone().unwrap().contains(&String::from("diffusers")));
        assert!(model.libraries.clone().unwrap().len() == 2);

        assert!(evaluate_rule(&model, &libraries_all_in).unwrap());
    }

    #[test]
    fn test_any_in() {
        let libraries_all_in = Rule {
            field_path: vec!["libraries".into()],
            operator: Operator::AnyIn,
            value: Value::Array(vec!["transformers".into()]),
        };

        let model = full_model();
        assert!(model.libraries.is_some());
        assert!(model.libraries.clone().unwrap().contains(&String::from("transformers")));
        assert!(model.libraries.clone().unwrap().contains(&String::from("diffusers")));
        assert!(model.libraries.clone().unwrap().len() == 2);

        assert!(evaluate_rule(&model, &libraries_all_in).unwrap());
    }

    #[test]
    fn test_none_in() {
        let libraries_all_in = Rule {
            field_path: vec!["libraries".into()],
            operator: Operator::NoneIn,
            value: Value::Array(vec!["foo".into(), "bar".into()]),
        };

        let model = full_model();
        assert!(model.libraries.is_some());
        assert!(model.libraries.clone().unwrap().contains(&String::from("transformers")));
        assert!(model.libraries.clone().unwrap().contains(&String::from("diffusers")));
        assert!(model.libraries.clone().unwrap().len() == 2);

        assert!(evaluate_rule(&model, &libraries_all_in).unwrap());
    }
}

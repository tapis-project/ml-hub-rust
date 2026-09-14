use serde_json::Value;

use super::evaluate_rule;
use crate::domain::entities::{
    deployment_strategy::rule_set::Rule, model::fixtures::full_external_model, operator::Operator,
};

#[test]
fn evaluates_provider_rule() {
    let rule = Rule {
        field_path: vec!["provider".into()],
        operator: Operator::Eq,
        value: Value::String("HuggingFace".into()),
    };

    let result = evaluate_rule(&full_external_model(), &rule);

    assert!(matches!(result, Ok(true)));
}

#[test]
fn evaluates_derived_name_rule() {
    let rule = Rule {
        field_path: vec!["metadata".into(), "derived".into(), "name".into()],
        operator: Operator::In,
        value: Value::Array(vec!["foo".into(), "bar".into()]),
    };

    let result = evaluate_rule(&full_external_model(), &rule);

    assert!(matches!(result, Ok(true)));
}

#[test]
fn evaluates_runtime_collection_rules() {
    let model = full_external_model();

    let contains = Rule {
        field_path: vec![
            "metadata".into(),
            "derived".into(),
            "inference_runtimes".into(),
        ],
        operator: Operator::Contains,
        value: Value::String("transformers".into()),
    };

    let all_in = Rule {
        field_path: vec![
            "metadata".into(),
            "derived".into(),
            "inference_runtimes".into(),
        ],
        operator: Operator::AllIn,
        value: Value::Array(vec!["transformers".into(), "diffusers".into()]),
    };

    assert!(matches!(evaluate_rule(&model, &contains), Ok(true)));
    assert!(matches!(evaluate_rule(&model, &all_in), Ok(true)));
}

#[test]
fn rejects_unknown_external_model_path() {
    let rule = Rule {
        field_path: vec!["canonical".into()],
        operator: Operator::Eq,
        value: Value::Null,
    };

    assert!(evaluate_rule(&full_external_model(), &rule).is_err());
}

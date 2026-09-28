use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde_json::{Value, json};

use super::*;
use crate::{
    ConditionEvaluationError, FieldPath, FieldValue, Operand, Operation, Operator, ResolveValue,
    StatementEvaluationError, ValueResolutionError,
};

static FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
struct JsonValues {
    values: HashMap<String, Value>,
}

#[derive(Debug)]
struct UndefinedValue;

impl JsonValues {
    fn new(values: impl IntoIterator<Item = (&'static str, Value)>) -> Self {
        Self {
            values: values
                .into_iter()
                .map(|(path, value)| (path.into(), value))
                .collect(),
        }
    }
}

impl ResolveValue for JsonValues {
    fn resolve_value(
        &self,
        field_path: Option<FieldPath>,
    ) -> Result<FieldValue, ValueResolutionError> {
        let field_path = field_path.ok_or_else(|| {
            ValueResolutionError::InvalidFieldPath("No field path provided".into())
        })?;

        let path = field_path.to_string();

        let value = self
            .values
            .get(&path)
            .cloned()
            .ok_or_else(|| ValueResolutionError::InvalidFieldPath(path))?;

        Ok(FieldValue::Json(value))
    }
}

impl ResolveValue for UndefinedValue {
    fn resolve_value(
        &self,
        field_path: Option<FieldPath>,
    ) -> Result<FieldValue, ValueResolutionError> {
        let field_path = field_path.ok_or_else(|| {
            ValueResolutionError::InvalidFieldPath("No field path provided".into())
        })?;

        if field_path.to_string() == "value" {
            return Ok(FieldValue::Undefined);
        }

        Err(ValueResolutionError::InvalidFieldPath(
            field_path.to_string(),
        ))
    }
}

fn real_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../deploy/k8s/site-configs/base/evaluations.json")
}

fn temporary_config(contents: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let id = FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "mlhub-evaluations-{}-{id}.json",
        std::process::id()
    ));

    fs::write(&path, contents)?;

    Ok(path)
}

fn valid_config() -> Value {
    json!({
        "evaluations": [{
            "name": "Check",
            "evaluation_strategy": "DNF",
            "parameters": ["item"],
            "expressions": [["Statement"]]
        }],
        "statements": [{
            "name": "Statement",
            "evaluation_strategy": "DNF",
            "parameters": ["item"],
            "conditions": [[{
                "operator": "Eq",
                "operands": [
                    {
                        "type": "parameter",
                        "parameter": "item",
                        "accessor": {"field_path": ["value"]}
                    },
                    {
                        "type": "literal",
                        "value": true
                    }
                ]
            }]]
        }]
    })
}

#[test]
fn treats_undefined_values_as_null_without_hiding_invalid_paths()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(Value::from(FieldValue::Undefined), Value::Null);

    let mut direct_null = valid_config();
    direct_null["statements"][0]["conditions"][0][0]["operands"][1]["value"] = Value::Null;
    let direct_null_path = temporary_config(&serde_json::to_string(&direct_null)?)?;

    let evaluator = Evaluator::load(&direct_null_path)?;

    let mut arguments = crate::Arguments::new();

    arguments.insert("item".into(), Rc::new(UndefinedValue));

    assert!(evaluator.evaluate("Check", &arguments)?);

    fs::remove_file(direct_null_path)?;

    let mut coalesced = valid_config();
    coalesced["statements"][0]["conditions"][0][0]["operands"][0]["operations"] = json!([{
        "operation": "coalesce",
        "coalesce": {"from": null, "to": true}
    }]);
    let coalesced_path = temporary_config(&serde_json::to_string(&coalesced)?)?;

    let evaluator = Evaluator::load(&coalesced_path)?;

    assert!(evaluator.evaluate("Check", &arguments)?);

    fs::remove_file(coalesced_path)?;

    let invalid_arguments = crate::Arguments::from([(
        "item".into(),
        Rc::new(JsonValues::new(Vec::<(&'static str, Value)>::new())) as Rc<dyn ResolveValue>,
    )]);
    let result = evaluator.evaluate("Check", &invalid_arguments);

    assert!(matches!(
        result,
        Err(EvaluatorError::Evaluation {
            source: StatementEvaluationError::Condition(ConditionEvaluationError::ValueResolution(
                ValueResolutionError::InvalidFieldPath(_)
            )),
            ..
        })
    ));

    Ok(())
}

#[test]
fn loads_the_complete_checked_in_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let evaluator = Evaluator::load(real_config_path())?;

    assert_eq!(evaluator.evaluations().len(), 4);
    assert_eq!(evaluator.statements().len(), 8);

    let trusted_model = evaluator
        .evaluations()
        .iter()
        .find(|evaluation| evaluation.name() == "Trusted Model")
        .ok_or("Trusted Model evaluation not found")?;

    assert_eq!(
        trusted_model.evaluation_type(),
        Some(crate::EvaluationType::ModelTrust)
    );
    let registered_trusted_authors = evaluator
        .statements()
        .iter()
        .find(|statement| statement.name() == "Trusted Authors")
        .ok_or("Trusted Authors statement not found")?;

    assert!(Rc::ptr_eq(
        &trusted_model.expressions()[0].statements()[0],
        registered_trusted_authors,
    ));

    let public = evaluator
        .statements()
        .iter()
        .find(|statement| statement.name() == "Public")
        .ok_or("Public statement not found")?;
    let public_left = public.conditions()[0][0].operands().left();

    assert!(matches!(
        public_left.operations(),
        [Operation::Coalesce {
            from: Value::Null,
            to: Value::Bool(false)
        }]
    ));

    let not_quantized = evaluator
        .statements()
        .iter()
        .find(|statement| statement.name() == "Not quantized")
        .ok_or("Not quantized statement not found")?;

    assert_eq!(
        not_quantized.conditions()[0][0].operator(),
        Operator::Pattern
    );
    assert!(not_quantized.conditions()[0][0].negate());
    assert!(not_quantized.conditions()[0][0].case_insensitive());

    let max_size = evaluator
        .statements()
        .iter()
        .find(|statement| statement.name() == "Lt Max Model Size")
        .ok_or("Lt Max Model Size statement not found")?;
    let size_operations = max_size.conditions()[0][1].operands().right().operations();

    assert_eq!(
        size_operations,
        [Operation::Multiply(vec![1_000_000_000.0, 0.6])]
    );

    Ok(())
}

#[test]
fn evaluates_by_name_and_evaluates_all_in_configuration_order()
-> Result<(), Box<dyn std::error::Error>> {
    let evaluator = Evaluator::load(real_config_path())?;
    let model = Rc::new(JsonValues::new([
        ("provider", json!("HuggingFace")),
        (
            "metadata/derived/inference_runtimes",
            json!(["transformers"]),
        ),
        ("metadata/derived/task_types", json!(["TextGeneration"])),
        ("metadata/derived/gated", json!(false)),
        ("metadata/derived/private", json!(false)),
        ("metadata/derived/size", json!(40_000_000_000_u64)),
        ("metadata/canonical/id", json!("Qwen/model")),
        ("metadata/canonical/tags", json!([])),
        ("metadata/canonical/config/quantization_config", Value::Null),
        ("metadata/canonical/gguf", Value::Null),
        ("metadata/canonical/author", json!("Qwen")),
    ]));
    let queue = Rc::new(JsonValues::new([(
        "hardware_profile/gpu/gpu_memory_gb",
        json!(80),
    )]));
    let mut arguments = crate::Arguments::new();

    arguments.insert("model".into(), model);
    arguments.insert("queue".into(), queue);

    assert!(evaluator.evaluate("FlexServ1.4 Compatibility", &arguments)?);
    assert!(evaluator.evaluate("Compatible Deployment Target", &arguments)?);
    assert!(!evaluator.evaluate("Trusted Model", &arguments)?);

    let result = evaluator.evaluate_all(&arguments)?;

    assert_eq!(
        result.passes(),
        [
            "FlexServ1.4 Compatibility",
            "FlexServ1.5 Compatibility",
            "Compatible Deployment Target"
        ]
    );
    assert_eq!(result.fails(), ["Trusted Model"]);

    Ok(())
}

#[test]
fn cpu_queue_fails_compatibility_without_attempting_multiply()
-> Result<(), Box<dyn std::error::Error>> {
    let evaluator = Evaluator::load(real_config_path())?;
    let model = Rc::new(JsonValues::new([(
        "metadata/derived/size",
        json!(1_000_000_u64),
    )]));
    let queue = Rc::new(JsonValues::new([(
        "hardware_profile/gpu/gpu_memory_gb",
        Value::Null,
    )]));
    let mut arguments = crate::Arguments::new();

    arguments.insert("model".into(), model);
    arguments.insert("queue".into(), queue);

    assert!(!evaluator.evaluate("Compatible Deployment Target", &arguments)?);

    Ok(())
}

#[test]
fn reports_unknown_evaluations_and_missing_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let evaluator = Evaluator::load(real_config_path())?;
    let arguments = crate::Arguments::new();

    assert!(matches!(
        evaluator.evaluate("Unknown", &arguments),
        Err(EvaluatorError::EvaluationNotFound(_))
    ));
    assert!(matches!(
        evaluator.evaluate("FlexServ1.4 Compatibility", &arguments),
        Err(EvaluatorError::MissingArgument { .. })
    ));

    Ok(())
}

#[test]
fn supports_cnf_and_case_insensitive_scalar_collection_operators()
-> Result<(), Box<dyn std::error::Error>> {
    let config = json!({
        "evaluations": [{
            "name": "CNF Check",
            "evaluation_strategy": "CNF",
            "parameters": ["item"],
            "expressions": [["Matches"], ["Pattern"]]
        }],
        "statements": [
            {
                "name": "Matches",
                "evaluation_strategy": "CNF",
                "parameters": ["item"],
                "conditions": [
                    [{
                        "operator": "AnyIn",
                        "case_insensitive": true,
                        "operands": [
                            {
                                "type": "parameter",
                                "parameter": "item",
                                "accessor": {"field_path": ["author"]}
                            },
                            {
                                "type": "literal",
                                "value": ["qwen", "openai"]
                            }
                        ]
                    }],
                    [{
                        "operator": "Eq",
                        "case_insensitive": true,
                        "operands": [
                            {
                                "type": "parameter",
                                "parameter": "item",
                                "accessor": {"field_path": ["author"]}
                            },
                            {
                                "type": "literal",
                                "value": "QWEN"
                            }
                        ]
                    }]
                ]
            },
            {
                "name": "Pattern",
                "evaluation_strategy": "DNF",
                "parameters": ["item"],
                "conditions": [[{
                    "operator": "Pattern",
                    "case_insensitive": true,
                    "operands": [
                        {
                            "type": "parameter",
                            "parameter": "item",
                            "accessor": {"field_path": ["name"]}
                        },
                        {
                            "type": "literal",
                            "value": ["^MODEL-[0-9]+$"]
                        }
                    ]
                }]]
            }
        ]
    });
    let path = temporary_config(&serde_json::to_string(&config)?)?;
    let evaluator = Evaluator::load(&path)?;
    let item = Rc::new(JsonValues::new([
        ("author", json!("Qwen")),
        ("name", json!("model-7")),
    ]));
    let mut arguments = crate::Arguments::new();

    arguments.insert("item".into(), item);

    assert!(evaluator.evaluate("CNF Check", &arguments)?);

    fs::remove_file(path)?;

    Ok(())
}

#[test]
fn rejects_invalid_configuration_graphs() -> Result<(), Box<dyn std::error::Error>> {
    let mut configurations = Vec::new();

    let mut duplicate = valid_config();
    let duplicated_statement = duplicate["statements"][0].clone();

    duplicate["statements"]
        .as_array_mut()
        .ok_or("statements is not an array")?
        .push(duplicated_statement);
    configurations.push(duplicate);

    let mut duplicate_evaluation = valid_config();
    let duplicated_evaluation = duplicate_evaluation["evaluations"][0].clone();

    duplicate_evaluation["evaluations"]
        .as_array_mut()
        .ok_or("evaluations is not an array")?
        .push(duplicated_evaluation);
    configurations.push(duplicate_evaluation);

    let mut missing_reference = valid_config();
    missing_reference["evaluations"][0]["expressions"] = json!([["Missing"]]);
    configurations.push(missing_reference);

    let mut duplicate_reference = valid_config();
    duplicate_reference["evaluations"][0]["expressions"] = json!([["Statement", "Statement"]]);
    configurations.push(duplicate_reference);

    let mut empty_expressions = valid_config();
    empty_expressions["evaluations"][0]["expressions"] = json!([]);
    configurations.push(empty_expressions);

    let mut incompatible_parameters = valid_config();
    incompatible_parameters["evaluations"][0]["parameters"] = json!([]);
    configurations.push(incompatible_parameters);

    let mut invalid_operands = valid_config();
    invalid_operands["statements"][0]["conditions"][0][0]["operands"] = json!([]);
    configurations.push(invalid_operands);

    let mut empty_condition_group = valid_config();
    empty_condition_group["statements"][0]["conditions"] = json!([[]]);
    configurations.push(empty_condition_group);

    let mut undeclared_parameter = valid_config();
    undeclared_parameter["statements"][0]["conditions"][0][0]["operands"][0]["parameter"] =
        json!("other");
    configurations.push(undeclared_parameter);

    let mut empty_field_path = valid_config();
    empty_field_path["statements"][0]["conditions"][0][0]["operands"][0]["accessor"]["field_path"] =
        json!([]);
    configurations.push(empty_field_path);

    let mut missing_literal = valid_config();

    missing_literal["statements"][0]["conditions"][0][0]["operands"][1]
        .as_object_mut()
        .ok_or("literal operand is not an object")?
        .remove("value");
    configurations.push(missing_literal);

    let mut invalid_pattern = valid_config();
    invalid_pattern["statements"][0]["conditions"][0][0]["operator"] = json!("Pattern");
    invalid_pattern["statements"][0]["conditions"][0][0]["operands"][1]["value"] = json!(["["]);
    configurations.push(invalid_pattern);

    let mut empty_multiply = valid_config();
    empty_multiply["statements"][0]["conditions"][0][0]["operands"][0]["operations"] =
        json!([{"operation": "multiply", "multiply": []}]);
    configurations.push(empty_multiply);

    for (index, configuration) in configurations.into_iter().enumerate() {
        let path = temporary_config(&serde_json::to_string(&configuration)?)?;
        let result = Evaluator::load(&path);

        assert!(
            matches!(
                result,
                Err(EvaluatorError::InvalidConfiguration(_))
                    | Err(EvaluatorError::MissingStatementReference { .. })
            ),
            "configuration {index} unexpectedly loaded"
        );

        fs::remove_file(path)?;
    }

    Ok(())
}

#[test]
fn distinguishes_file_deserialization_and_empty_configuration_errors()
-> Result<(), Box<dyn std::error::Error>> {
    let missing = std::env::temp_dir().join("mlhub-evaluations-file-does-not-exist.json");

    assert!(matches!(
        Evaluator::load(missing),
        Err(EvaluatorError::FileRead { .. })
    ));

    let malformed = temporary_config("{")?;

    assert!(matches!(
        Evaluator::load(&malformed),
        Err(EvaluatorError::Deserialization { .. })
    ));

    fs::remove_file(malformed)?;

    let empty = temporary_config("   ")?;

    assert!(matches!(
        Evaluator::load(&empty),
        Err(EvaluatorError::InvalidConfiguration(_))
    ));

    fs::remove_file(empty)?;

    Ok(())
}

#[test]
fn rejects_unknown_fields_and_mismatched_discriminators() -> Result<(), Box<dyn std::error::Error>>
{
    let mut unknown = valid_config();
    unknown["statements"][0]["unexpected"] = json!(true);
    let unknown_path = temporary_config(&serde_json::to_string(&unknown)?)?;

    assert!(matches!(
        Evaluator::load(&unknown_path),
        Err(EvaluatorError::Deserialization { .. })
    ));

    fs::remove_file(unknown_path)?;

    let mut mismatch = valid_config();
    mismatch["statements"][0]["conditions"][0][0]["operands"][0]["operations"] = json!([{
        "operation": "multiply",
        "coalesce": {"from": null, "to": false}
    }]);
    let mismatch_path = temporary_config(&serde_json::to_string(&mismatch)?)?;

    assert!(matches!(
        Evaluator::load(&mismatch_path),
        Err(EvaluatorError::InvalidConfiguration(_))
    ));

    fs::remove_file(mismatch_path)?;

    Ok(())
}

#[test]
fn applies_operations_in_order() -> Result<(), Box<dyn std::error::Error>> {
    let operations = [
        Operation::Coalesce {
            from: Value::Null,
            to: json!(10),
        },
        Operation::Multiply(vec![2.0, 0.5]),
    ];
    let mut value = Value::Null;

    for operation in operations {
        value = operation.apply(value)?;
    }

    assert_eq!(value, json!(10.0));

    Ok(())
}

#[test]
fn evaluates_every_operator() -> Result<(), Box<dyn std::error::Error>> {
    assert!(Operator::Eq.evaluate(&json!(1), &json!(1))?);
    assert!(Operator::Neq.evaluate(&json!(1), &json!(2))?);
    assert!(Operator::Gte.evaluate(&json!(2), &json!(2))?);
    assert!(Operator::Lte.evaluate(&json!(2), &json!(2))?);
    assert!(Operator::Gt.evaluate(&json!(3), &json!(2))?);
    assert!(Operator::Lt.evaluate(&json!(2), &json!(3))?);
    assert!(Operator::In.evaluate(&json!("a"), &json!(["a", "b"]))?);
    assert!(Operator::Contains.evaluate(&json!(["a", "b"]), &json!("b"))?);
    assert!(Operator::NotIn.evaluate(&json!("c"), &json!(["a", "b"]))?);
    assert!(Operator::AnyIn.evaluate(&json!("a"), &json!(["a", "b"]))?);
    assert!(Operator::AllIn.evaluate(&json!(["a", "b"]), &json!(["b", "a"]))?);
    assert!(Operator::NoneIn.evaluate(&json!(["c"]), &json!(["a", "b"]))?);
    assert!(Operator::Pattern.evaluate_with_options(
        &json!("MODEL-7"),
        &json!(["^model-[0-9]+$"]),
        true,
    )?);

    Ok(())
}

#[test]
fn exposes_literal_operand_values() -> Result<(), Box<dyn std::error::Error>> {
    let evaluator = Evaluator::load(real_config_path())?;
    let statement = evaluator
        .statements()
        .iter()
        .find(|statement| statement.name() == "From HuggingFace")
        .ok_or("From HuggingFace statement not found")?;
    let right = statement.conditions()[0][0].operands().right();

    let Operand::Literal(literal) = right else {
        return Err("expected literal operand".into());
    };

    assert_eq!(literal.value(), &json!("HuggingFace"));

    Ok(())
}

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FieldPath(Vec<String>);

impl FieldPath {
    pub fn new(field_path: Vec<String>) -> Self {
        Self(field_path)
    }

    pub fn into_inner(&self) -> &[String] {
        &self.0
    }
}

impl std::fmt::Display for FieldPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_value = String::new();
        for part in self.into_inner().into_iter() {
            display_value = format!("{}/{}", display_value, part);
        }

        write!(
            f,
            "{}",
            display_value
                .strip_prefix("/")
                .unwrap_or(display_value.as_str())
        )
    }
}

#[derive(Clone, Debug)]
pub enum FieldValue {
    Undefined,
    String(Option<String>),
    Strings(Vec<String>),
    Boolean(bool),
    Unsigned(u64),
    OptionalUnsigned(Option<u128>),
    Json(Value),
}

impl From<FieldValue> for Value {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::Undefined => Value::Null,
            FieldValue::String(value) => serde_json::to_value(value).unwrap_or(Value::Null),
            FieldValue::Strings(value) => serde_json::to_value(value).unwrap_or(Value::Null),
            FieldValue::Boolean(value) => Value::Bool(value),
            FieldValue::Unsigned(value) => Value::Number(value.into()),
            FieldValue::OptionalUnsigned(value) => {
                serde_json::to_value(value).unwrap_or(Value::Null)
            }
            FieldValue::Json(value) => value,
        }
    }
}

use std::{collections::HashMap, panic::AssertUnwindSafe};
use serde::{Serialize, Serializer, Deserialize};
use serde_json::{Value, Map};

#[derive(Debug, Clone, Serialize)]
pub struct JsonObject {
    inner: Map<String, Value>,
}

impl JsonObject {
    pub fn new() -> Self {
        return JsonObject {
            inner: Map::new()
        }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.inner.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    pub fn into_hashmap(self) -> HashMap<String, Value> {
        self.inner.into_iter().collect()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct JsonValue {
    inner: Value,
}

impl JsonValue {
    pub fn new(value: Value) -> Self {
        JsonValue { inner: value }
    }
}

impl Serialize for JsonValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Delegate serialization to the `inner` field
        self.inner.serialize(serializer)
    }
}
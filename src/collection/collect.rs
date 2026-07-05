use crate::lib::Value;

// Collects the iterable "values" of a collection: array elements, object
// values, or string characters. Non-collections yield an empty list
pub(crate) fn collection_values(v: &Value) -> Vec<Value> {
    match v {
        Value::Array(vec) => vec.clone(),
        Value::Object(o) => o.values().cloned().collect(),
        Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
        _ => vec![],
    }
}

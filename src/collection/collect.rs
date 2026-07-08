use crate::lib::Value;

// Collects the iterable "values" of a collection: array elements, object
// values, or string characters. Non-collections yield an empty list.
// Consuming on purpose: arrays are moved out for free instead of cloned;
// borrow-iterate inline (like each/includes do) when the original is needed
pub(crate) fn collection_values(v: Value) -> Vec<Value> {
    match v {
        Value::Array(vec) => vec,
        Value::Object(o) => o.into_iter().map(|(_, v)| v).collect(),
        Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
        _ => vec![],
    }
}

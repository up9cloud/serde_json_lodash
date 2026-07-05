use crate::lib::Value;

fn deep(v: &Value, customizer: fn(&Value) -> Option<Value>) -> Value {
    if let Some(result) = customizer(v) {
        return result;
    }
    match v {
        Value::Array(vec) => Value::Array(vec.iter().map(|i| deep(i, customizer)).collect()),
        Value::Object(o) => Value::Object(
            o.iter()
                .map(|(k, i)| (k.clone(), deep(i, customizer)))
                .collect(),
        ),
        _ => v.clone(),
    }
}

/// See lodash [cloneDeepWith](https://lodash.com/docs/#cloneDeepWith)
///
/// `customizer` is invoked recursively for every value; returning `None`
/// falls back to the default deep clone behavior
pub fn clone_deep_with(v: &Value, customizer: fn(&Value) -> Option<Value>) -> Value {
    deep(v, customizer)
}

/// Based on [clone_deep_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// use serde_json::Value;
/// fn customizer(v: &Value) -> Option<Value> {
///   if v.is_number() { Some(json!(0)) } else { None }
/// }
/// assert_eq!(
///   clone_deep_with!(&json!({"a": [1, 2]}), customizer),
///   json!({"a": [0, 0]})
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(clone_deep_with!(), json!(null));
/// assert_eq!(clone_deep_with!(&json!([1])), json!([1]));
/// ```
#[macro_export]
macro_rules! clone_deep_with {
    () => {
        json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::clone_deep($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::clone_deep_with($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::clone_deep_with($a, $b)
    };
}

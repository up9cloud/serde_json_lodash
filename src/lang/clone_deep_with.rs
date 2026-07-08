use crate::lib::Value;

fn deep(v: &Value, customizer: &impl Fn(&Value) -> Option<Value>) -> Value {
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

/// Fn form of [clone_deep_with!](crate::clone_deep_with!); see it for the full docs
///
/// `_x` form: **not provided** — see [clone_deep_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::clone_deep_with;
/// # use serde_json::json;
/// assert_eq!(clone_deep_with(&json!([1]), |_| None), json!([1]));
/// ```
pub fn clone_deep_with(v: &Value, customizer: impl Fn(&Value) -> Option<Value>) -> Value {
    deep(v, &customizer)
}

/// See lodash [cloneDeepWith](https://lodash.com/docs/#cloneDeepWith)
///
/// `customizer` is invoked recursively for every value; returning `None`
/// falls back to the default deep clone behavior
///
/// Fn form: [clone_deep_with()] | `_x` form: **not provided** — see [clone_deep_with_x()]
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
///   clone_deep_with!(json!({"a": [1, 2]}), customizer),
///   json!({"a": [0, 0]})
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(clone_deep_with!(), json!(null));
/// assert_eq!(clone_deep_with!(json!([1])), json!([1]));
/// ```
#[macro_export]
macro_rules! clone_deep_with {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::clone_deep(&$a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::clone_deep_with(&$a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::clone_deep_with(&$a, $b)
    };
}

build_not_provided_x!(clone_deep_with, clone_deep_with_x);

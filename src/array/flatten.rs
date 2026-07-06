use crate::lib::{json, Value};

// internal worker for [flatten()].
fn x_flatten_x(vec: Vec<Value>) -> Vec<Value> {
    if vec.is_empty() {
        return vec![];
    }
    let mut result = vec![];
    for item in vec.into_iter() {
        match item {
            Value::Null
            | Value::Bool(_)
            | Value::String(_)
            | Value::Number(_)
            | Value::Object(_) => {
                result.push(item);
            }
            Value::Array(vec) => {
                for v in vec.into_iter() {
                    result.push(v);
                }
            }
        }
    }
    result
}

/// See lodash [flatten](https://lodash.com/docs/#flatten)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::flatten;
/// # use serde_json::json;
/// assert_eq!(flatten(json!([1, [2, [3, [4]], 5]])), json!([1, 2, [3, [4]], 5]));
/// ```
pub fn flatten<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    match v {
        Value::Null => json!([]),
        Value::Bool(_) => json!([]),
        Value::String(s) => {
            if s.is_empty() {
                return json!([]);
            }
            Value::Array(
                s.chars()
                    .map(|c| Value::String(c.to_string()))
                    .collect::<Vec<Value>>(),
            )
        }
        Value::Number(_) => json!([]),
        Value::Array(vec) => Value::Array(x_flatten_x(vec)),
        Value::Object(_) => json!([]),
    }
}

/// Based on [flatten()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   flatten!(json!([1, [2, [3, [4]], 5]])),
///   json!([1, 2, [3, [4]], 5])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(flatten!(), json!([]));
/// assert_eq!(flatten!(json!(null)), json!([]));
/// assert_eq!(flatten!(json!(false)), json!([]));
/// assert_eq!(flatten!(json!(0)), json!([]));
/// assert_eq!(flatten!(json!("")), json!([]));
/// assert_eq!(flatten!(json!("ab")), json!(["a","b"]));
/// assert_eq!(flatten!(json!("りしれ")), json!(["り","し","れ"]));
/// assert_eq!(flatten!(json!({})), json!([]));
/// assert_eq!(flatten!(json!({"a":1})), json!([]));
/// assert_eq!(flatten!(json!([null,false,0,"",[null,[false]],{"a":1}])), json!([null,false,0,"",null,[false],{"a":1}]));
/// ```
#[macro_export]
macro_rules! flatten {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::flatten($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::flatten($a)
    };
}

/// `_x` helper for [flatten()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [flatten()] and read the returned `Value`.
pub fn flatten_x() {
    todo!()
}
/// Based on [flatten_x()]
#[macro_export]
macro_rules! flatten_x {
    ($($t:tt)*) => {
        $crate::flatten_x()
    };
}

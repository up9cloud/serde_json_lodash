use crate::lib::{json, Value};

/// `x_`/`_x` helper for [flatten()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_flatten_x;
/// # use serde_json::json;
/// assert_eq!(x_flatten_x(vec![json!(1), json!([2, 3])]), vec![json!(1), json!(2), json!(3)]);
/// ```
pub fn x_flatten_x(vec: Vec<Value>) -> Vec<Value> {
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
pub fn flatten(v: Value) -> Value {
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

/// Based on [x_flatten_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_flatten_x!(vec![json!(1), json!([2, 3])]), vec![json!(1), json!(2), json!(3)]);
/// ```
#[macro_export]
macro_rules! x_flatten_x {
    () => {
        $crate::x_flatten_x(vec![])
    };
    ($a:expr $(,)*) => {
        $crate::x_flatten_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_flatten_x($a)
    };
}

use crate::lib::{json, Value};

/// `x_`/`_x` helper for [flatten_deep()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_flatten_deep_x;
/// # use serde_json::json;
/// assert_eq!(x_flatten_deep_x(vec![json!(1), json!([2, [3]])]), vec![json!(1), json!(2), json!(3)]);
/// ```
pub fn x_flatten_deep_x(vec: Vec<Value>) -> Vec<Value> {
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
                for item in x_flatten_deep_x(vec).into_iter() {
                    result.push(item);
                }
            }
        }
    }
    result
}
/// See lodash [flattenDeep](https://lodash.com/docs/#flattenDeep)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::flatten_deep;
/// # use serde_json::json;
/// assert_eq!(flatten_deep(json!([1, [2, [3, [4]], 5]])), json!([1, 2, 3, 4, 5]));
/// ```
pub fn flatten_deep(v: Value) -> Value {
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
        Value::Array(vec) => Value::Array(x_flatten_deep_x(vec)),
        Value::Object(_) => json!([]),
    }
}
/// Based on [flatten_deep()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   flatten_deep!(json!([1, [2, [3, [4]], 5]])),
///   json!([1, 2, 3, 4, 5])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(flatten_deep!(), json!([]));
/// assert_eq!(flatten_deep!(json!(null)), json!([]));
/// assert_eq!(flatten_deep!(json!(false)), json!([]));
/// assert_eq!(flatten_deep!(json!(0)), json!([]));
/// assert_eq!(flatten_deep!(json!("")), json!([]));
/// assert_eq!(flatten_deep!(json!("ab")), json!(["a","b"]));
/// assert_eq!(flatten_deep!(json!("りしれ")), json!(["り","し","れ"]));
/// assert_eq!(flatten_deep!(json!({})), json!([]));
/// assert_eq!(flatten_deep!(json!({"a":1})), json!([]));
/// assert_eq!(flatten_deep!(json!([null,false,0,"",[null,[false]],{"a":1}])), json!([null,false,0,"",null,false,{"a":1}]));
/// ```
#[macro_export]
macro_rules! flatten_deep {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::flatten_deep($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::flatten_deep($a)
    };
}

/// Based on [x_flatten_deep_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_flatten_deep_x!(vec![json!(1), json!([2, [3]])]), vec![json!(1), json!(2), json!(3)]);
/// ```
#[macro_export]
macro_rules! x_flatten_deep_x {
    () => {
        $crate::x_flatten_deep_x(vec![])
    };
    ($a:expr $(,)*) => {
        $crate::x_flatten_deep_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_flatten_deep_x($a)
    };
}

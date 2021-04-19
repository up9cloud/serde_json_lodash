use crate::lib::{json, Value};

///
pub fn x_flatten_deep_x(vec: Vec<Value>) -> Vec<Value> {
    if vec.len() == 0 {
        return vec![]
    }
    let mut result = vec![];
    for item in vec.into_iter() {
        match item {
            Value::Null |
            Value::Bool(_) |
            Value::String(_) |
            Value::Number(_) |
            Value::Object(_) => {
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
///
pub fn flatten_deep(v: Value) -> Value {
    match v {
        Value::Null => json!([]),
        Value::Bool(_) => json!([]),
        Value::String(s) => {
            if s == "" {
                return json!([])
            }
            Value::Array(
                s.chars()
                    .map(|c| Value::String(c.to_string()))
                    .collect::<Vec<Value>>()
            )
        },
        Value::Number(_) => json!([]),
        Value::Array(vec) => Value::Array(x_flatten_deep_x(vec)),
        Value::Object(_) => json!([])
    }
}
/// See [lodash flattenDeep](https://lodash.com/docs/#flattenDeep)
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
/// More examples:
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
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::flatten_deep($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::flatten_deep($a)
    };
}

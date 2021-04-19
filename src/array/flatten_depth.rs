use crate::lib::{json, Value};

///
pub fn x_flatten_depth_x(vec: Vec<Value>, depth: usize) -> Vec<Value> {
    if depth == 0 {
        return vec;
    }
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
                for item in x_flatten_depth_x(vec, depth - 1).into_iter() {
                    result.push(item);
                }
            }
        }
    }
    result
}
///
pub fn flatten_depth(v: Value, depth: usize) -> Value {
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
        Value::Array(vec) => Value::Array(x_flatten_depth_x(vec, depth)),
        Value::Object(_) => json!([]),
    }
}

/// See [lodash flattenDepth](https://lodash.com/docs/#flattenDepth)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([1, [2, [3, [4]], 5]]);
/// assert_eq!(
///   flatten_depth!(array.clone(), 1),
///   json!([1, 2, [3, [4]], 5])
/// );
/// assert_eq!(
///   flatten_depth!(array.clone(), 2),
///   json!([1, 2, 3, [4], 5])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(flatten_depth!(), json!([]));
/// assert_eq!(flatten_depth!(json!(null)), json!([]));
/// assert_eq!(flatten_depth!(json!(false)), json!([]));
/// assert_eq!(flatten_depth!(json!(0)), json!([]));
/// assert_eq!(flatten_depth!(json!("")), json!([]));
/// assert_eq!(flatten_depth!(json!("ab")), json!(["a","b"]));
/// assert_eq!(flatten_depth!(json!("りしれ")), json!(["り","し","れ"]));
/// assert_eq!(flatten_depth!(json!({})), json!([]));
/// assert_eq!(flatten_depth!(json!({"a":1})), json!([]));
/// assert_eq!(flatten_depth!(json!([null,false,0,"",[null,[false]],{"a":1}])), json!([null,false,0,"",null,[false],{"a":1}]));
/// assert_eq!(flatten_depth!(json!([null,false,0,"",[null,[false]],{"a":1}]), 100), json!([null,false,0,"",null,false,{"a":1}]));
/// assert_eq!(flatten_depth!(json!([[[null]]]), 2), json!([null]));
/// ```
#[macro_export]
macro_rules! flatten_depth {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::flatten_depth($a, 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::flatten_depth($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::flatten_depth($a, $b)
    };
}

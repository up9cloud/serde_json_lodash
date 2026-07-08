use crate::lib::{Value, json};

// internal worker for [flatten_depth()].
fn x_flatten_depth_x(vec: Vec<Value>, depth: usize) -> Vec<Value> {
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

/// Fn form of [flatten_depth!](crate::flatten_depth!); see it for the full docs
///
/// `_x` form: **not provided** — see [flatten_depth_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::flatten_depth;
/// # use serde_json::json;
/// assert_eq!(flatten_depth(json!([null,false,0,"",[null,[false]],{"a":1}]), 100), json!([null,false,0,"",null,false,{"a":1}]));
/// ```
pub fn flatten_depth<A: Into<Value>>(v: A, depth: usize) -> Value {
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
        Value::Array(vec) => Value::Array(x_flatten_depth_x(vec, depth)),
        Value::Object(_) => json!([]),
    }
}

/// See lodash [flattenDepth](https://lodash.com/docs/#flattenDepth)
///
/// Fn form: [flatten_depth()] | `_x` form: **not provided** — see [flatten_depth_x()]
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
/// Additional cases:
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
        $crate::lib::json!([])
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

build_not_provided_x!(flatten_depth, flatten_depth_x);

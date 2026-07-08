use crate::lib::{Value, json};

/// Fn form of [initial!](crate::initial!); see it for the full docs
///
/// `_x` form: **not provided** — see [initial_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::initial;
/// # use serde_json::json;
/// assert_eq!(initial(json!([1, 2, 3])), json!([1, 2]));
/// ```
pub fn initial(v: Value) -> Value {
    match v {
        Value::Null => json!([]),
        Value::Bool(_) => json!([]),
        Value::String(s) => {
            let mut vec = s.chars().map(|x| json!(x)).collect::<Vec<Value>>();
            if vec.len() <= 1 {
                return json!([]);
            }
            vec.pop();
            Value::Array(vec)
        }
        Value::Number(_) => json!([]),
        Value::Array(mut vec) => {
            if vec.len() <= 1 {
                return json!([]);
            }
            vec.pop();
            Value::Array(vec)
        }
        Value::Object(_) => json!([]),
    }
}

/// See lodash [initial](https://lodash.com/docs/#initial)
///
/// Fn form: [initial()] | `_x` form: **not provided** — see [initial_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   initial!(json!([1, 2, 3])),
///   json!([1, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(initial!(), json!([]));
/// assert_eq!(initial!(json!(null)), json!([]));
/// assert_eq!(initial!(json!(false)), json!([]));
/// assert_eq!(initial!(json!(0)), json!([]));
/// assert_eq!(initial!(json!("")), json!([]));
/// assert_eq!(initial!(json!("abc")), json!(["a","b"]));
/// assert_eq!(initial!(json!("日本国")), json!(["日","本"]));
/// assert_eq!(initial!(json!({})), json!([]));
/// assert_eq!(initial!(json!({"a":1})), json!([]));
/// ```
#[macro_export]
macro_rules! initial {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::initial($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::initial($a)
    };
}

build_not_provided_x!(initial, initial_x);

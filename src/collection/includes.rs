use crate::lib::{Value, json};

use crate::internal::resolve_from_index;

/// Fn form of [includes!](crate::includes!); see it for the full docs
///
/// `_x` forms: [includes_x!](crate::includes_x!), [includes_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::includes;
/// # use serde_json::json;
/// assert_eq!(includes(&json!([1, 2, 3]), &json!(1), 0), json!(true));
/// ```
pub fn includes(collection: &Value, value: &Value, from_index: isize) -> Value {
    json!(includes_x(collection, value, from_index))
}

/// See lodash [includes](https://lodash.com/docs/#includes)
///
/// Fn form: [includes()] | `_x` forms: [includes_x!](crate::includes_x!), [includes_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(includes!(json!([1, 2, 3]), json!(1)), json!(true));
/// assert_eq!(includes!(json!([1, 2, 3]), json!(1), 2), json!(false));
/// assert_eq!(includes!(json!({ "a": 1, "b": 2 }), json!(1)), json!(true));
/// assert_eq!(includes!(json!("abcd"), json!("bc")), json!(true));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(includes!(), json!(false));
/// assert_eq!(includes!(json!(null)), json!(false));
/// assert_eq!(includes!(json!({"a": 1})), json!(false));
/// assert_eq!(includes!(json!(null), json!(null)), json!(false));
/// assert_eq!(includes!(json!(1), json!(1)), json!(false));
/// assert_eq!(includes!(json!(1), json!(2)), json!(false));
/// assert_eq!(includes!(json!([1, 2, 3]), json!(2)), json!(true));
/// assert_eq!(includes!(json!("abc"), json!("bc")), json!(true));
/// assert_eq!(includes!(json!([1, 2, 3]), json!(9)), json!(false));
/// // negative fromIndex counts back from the end
/// assert_eq!(includes!(json!([1, 2, 3]), json!(3), -1), json!(true));
/// assert_eq!(includes!(json!([1, 2, 3]), json!(1), -9), json!(true));
/// // fromIndex applies to an object's values sequence too
/// assert_eq!(includes!(json!({"a": 1, "b": 2}), json!(1), 1), json!(false));
/// assert_eq!(includes!(json!({"a": 1, "b": 2}), json!(2), 1), json!(true));
/// // and to string search, by char offset
/// assert_eq!(includes!(json!("abcd"), json!("bc"), 1), json!(true));
/// assert_eq!(includes!(json!("abcd"), json!("bc"), 2), json!(false));
/// assert_eq!(includes!(json!("abcd"), json!("cd"), -2), json!(true));
/// ```
#[macro_export]
macro_rules! includes {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::includes(&$a, &$b, 0)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::includes(&$a, &$b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::includes(&$a, &$b, $c)
    };
}

/// `_x` helper for [includes!](crate::includes!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [includes_x!](crate::includes_x!) | `Value` forms: [includes!](crate::includes!), [includes()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::includes_x;
/// # use serde_json::json;
/// assert_eq!(includes_x(&json!([1, 2, 3]), &json!(1), 0), true);
/// ```
pub fn includes_x(collection: &Value, value: &Value, from_index: isize) -> bool {
    match collection {
        Value::String(s) => match value {
            Value::String(sub) => {
                if from_index == 0 {
                    return s.contains(sub.as_str());
                }
                let start = resolve_from_index(s.chars().count(), from_index);
                match s.char_indices().nth(start) {
                    Some((byte, _)) => s[byte..].contains(sub.as_str()),
                    None => false,
                }
            }
            _ => false,
        },
        Value::Array(vec) => {
            let start = resolve_from_index(vec.len(), from_index);
            vec.iter().skip(start).any(|v| v == value)
        }
        // like lodash, fromIndex applies to the object's values sequence
        Value::Object(o) => {
            let start = resolve_from_index(o.len(), from_index);
            o.values().skip(start).any(|v| v == value)
        }
        _ => false,
    }
}

/// `_x` helper for [includes!](crate::includes!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [includes_x()] | `Value` forms: [includes!](crate::includes!), [includes()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(includes_x!(json!([1, 2, 3]), json!(1)), true);
/// assert_eq!(includes_x!(json!([1, 2, 3]), json!(1), 2), false);
/// ```
#[macro_export]
macro_rules! includes_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::includes_x(&$a, &$b, 0)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::includes_x(&$a, &$b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::includes_x(&$a, &$b, $c)
    };
}

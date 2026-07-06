use crate::lib::{json, Value};
use crate::to_path_x;

/// `_x` helper for [has()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::has_x;
/// # use serde_json::json;
/// assert_eq!(has_x(&json!({"a": {"b": 2}}), json!("a.b")), true);
/// ```
pub fn has_x(object: &Value, path: Value) -> bool {
    let p_vec = to_path_x(path);
    if p_vec.is_empty() {
        return false;
    }
    let mut cur = object;
    for k in p_vec.iter() {
        match cur {
            Value::Object(o) => match o.get(k) {
                Some(v) => cur = v,
                None => return false,
            },
            Value::Array(vec) => match k.parse::<usize>() {
                Ok(i) if i < vec.len() => cur = &vec[i],
                _ => return false,
            },
            _ => return false,
        }
    }
    true
}
/// See lodash [has](https://lodash.com/docs/#has)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::has;
/// # use serde_json::json;
/// assert_eq!(has(&json!({"a": {"b": 2}}), json!("a.b")), json!(true));
/// ```
pub fn has(object: &Value, path: Value) -> Value {
    json!(has_x(object, path))
}

/// Based on [has_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(has_x!(&json!({"a": {"b": 2}}), json!("a.b")), true);
/// ```
#[macro_export]
macro_rules! has_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::has_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::has_x($a, $b)
    };
}
/// Based on [has()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(has!(), json!(false));
/// assert_eq!(has!(&json!({"a": 1})), json!(false));
/// assert_eq!(has!(&json!({"a": [{"b": 3}]}), json!("a[0].b")), json!(true));
/// ```
#[macro_export]
macro_rules! has {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::has($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::has($a, $b)
    };
}

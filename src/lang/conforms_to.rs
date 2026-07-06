use crate::lib::{json, Value};

/// A `(key, predicate)` pair for [conforms_to()].
pub type Conform<'a> = (&'a str, fn(&Value) -> bool);

/// `_x` helper for [conforms_to()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::conforms_to_x;
/// # use serde_json::json;
/// assert_eq!(conforms_to_x(&json!({}), vec![("a", |_: &serde_json::Value| true)]), false);
/// ```
pub fn conforms_to_x(object: &Value, source: Vec<Conform>) -> bool {
    for (key, predicate) in source {
        match object.get(key) {
            Some(v) => {
                if !predicate(v) {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}
/// See lodash [conformsTo](https://lodash.com/docs/#conformsTo)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::conforms_to;
/// # use serde_json::json;
/// assert_eq!(conforms_to(&json!({}), vec![("a", |_: &serde_json::Value| true)]), json!(false));
/// ```
pub fn conforms_to(object: &Value, source: Vec<Conform>) -> Value {
    json!(conforms_to_x(object, source))
}

/// Based on [conforms_to_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(conforms_to_x!(&json!({}), vec![("a", |_: &serde_json::Value| true)]), false);
/// ```
#[macro_export]
macro_rules! conforms_to_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::conforms_to_x($a, vec![])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::conforms_to_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::conforms_to_x($a, $b)
    };
}
/// Based on [conforms_to()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(conforms_to!(), json!(true));
/// assert_eq!(conforms_to!(&json!({})), json!(true));
/// assert_eq!(conforms_to!(&json!({}), vec![("a", |_: &serde_json::Value| true)]), json!(false));
/// ```
#[macro_export]
macro_rules! conforms_to {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::conforms_to($a, vec![])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::conforms_to($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::conforms_to($a, $b)
    };
}

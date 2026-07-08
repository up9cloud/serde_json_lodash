use crate::lib::{Value, json};

/// Fn form of [find_key!](crate::find_key!); see it for the full docs
///
/// `_x` form: **not provided** — see [find_key_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_key;
/// # use serde_json::json;
/// assert_eq!(find_key(&json!({"a": 1, "b": 2}), |v| v == &json!(2)), json!("b"));
/// ```
pub fn find_key(object: &Value, predicate: impl Fn(&Value) -> bool) -> Value {
    if let Value::Object(o) = object {
        for (k, v) in o {
            if predicate(v) {
                return json!(k);
            }
        }
    }
    Value::Null
}

/// See lodash [findKey](https://lodash.com/docs/#findKey)
///
/// Returns the key of the first value matching `predicate`, else `Null`
///
/// Fn form: [find_key()] | `_x` form: **not provided** — see [find_key_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!({
///   "barney":  { "age": 36, "active": true },
///   "fred":    { "age": 40, "active": false }
/// });
/// assert_eq!(
///   find_key!(&users, |o| o["active"] == json!(false)),
///   json!("fred")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_key!(), json!(null));
/// assert_eq!(find_key!(json!({"a": 1})), json!(null));
/// assert_eq!(find_key!(json!({"a": 1, "b": 2}), |v| v == &json!(2)), json!("b"));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(find_key!(json!({"x": {"a": 0, "b": 1}, "y": {"a": 2, "b": 1}}), json!({"b": 1})), json!("x"));
/// assert_eq!(find_key!(json!({"x": {"a": 0, "b": 1}, "y": {"a": 2, "b": 1}}), "a"), json!("y"));
/// ```
#[macro_export]
macro_rules! find_key {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(null)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::find_key(&$a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::find_key(&$a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::find_key(&$a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::find_key(&$a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::find_key(&$a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::find_key(&$a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find_key(&$a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::find_key(&$a, $b)
    };
}

build_not_provided_x!(find_key, find_key_x);

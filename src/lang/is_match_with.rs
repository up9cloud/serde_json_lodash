use crate::lib::{json, Value};
use crate::internal::base_is_match;

/// `_x` helper for [is_match_with()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_match_with_x;
/// # use serde_json::json;
/// assert_eq!(is_match_with_x(&json!({"a": 1}), &json!({"a": 1}), |_, _| None), true);
/// ```
pub fn is_match_with_x(
    object: &Value,
    source: &Value,
    customizer: fn(&Value, &Value) -> Option<bool>,
) -> bool {
    match source {
        Value::Object(so) => match object {
            Value::Object(oo) => so.iter().all(|(k, sv)| match oo.get(k) {
                Some(ov) => match customizer(ov, sv) {
                    Some(result) => result,
                    None => base_is_match(ov, sv),
                },
                None => false,
            }),
            _ => so.is_empty(),
        },
        _ => match customizer(object, source) {
            Some(result) => result,
            None => base_is_match(object, source),
        },
    }
}
/// See lodash [isMatchWith](https://lodash.com/docs/#isMatchWith)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_match_with;
/// # use serde_json::json;
/// assert_eq!(is_match_with(&json!({"a": 1}), &json!({"a": 1}), |_, _| None), json!(true));
/// ```
pub fn is_match_with(
    object: &Value,
    source: &Value,
    customizer: fn(&Value, &Value) -> Option<bool>,
) -> Value {
    json!(is_match_with_x(object, source, customizer))
}

/// Based on [is_match_with_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_match_with_x!(&json!({"a": 1}), &json!({"a": 1}), |_, _| None), true);
/// ```
#[macro_export]
macro_rules! is_match_with_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        true
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_match($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::is_match_with_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::is_match_with_x($a, $b, $c)
    };
}
/// Based on [is_match_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_match_with!(), json!(true));
/// assert_eq!(is_match_with!(&json!({"a": 1})), json!(true));
/// assert_eq!(is_match_with!(&json!({"a": 1}), &json!({"a": 1})), json!(true));
/// ```
#[macro_export]
macro_rules! is_match_with {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(true)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_match($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::is_match_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::is_match_with($a, $b, $c)
    };
}

use crate::lib::{json, Value};
use super::pad::make_padding;

/// `x_`/`_x` helper for [pad_start()]: takes a primitive argument and returns a primitive value.
pub fn x_pad_start_x(s: &str, length: usize, chars: &str) -> String {
    let len = s.chars().count();
    if length <= len || chars.is_empty() {
        return s.into();
    }
    let mut out = make_padding(length - len, chars);
    out.push_str(s);
    out
}
/// See lodash [padStart](https://lodash.com/docs/#padStart)
pub fn pad_start(v: Value, length: usize, chars: &str) -> Value {
    json!(x_pad_start_x(&crate::to_string_x(v), length, chars))
}

/// Based on [pad_start()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   pad_start!(json!("abc"), 6),
///   json!("   abc")
/// );
/// assert_eq!(
///   pad_start!(json!("abc"), 6, "_-"),
///   json!("_-_abc")
/// );
/// assert_eq!(
///   pad_start!(json!("abc"), 3),
///   json!("abc")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pad_start!(), json!(""));
/// assert_eq!(pad_start!(json!("abc")), json!("abc"));
/// assert_eq!(pad_start!(json!(6), 3, "0"), json!("006"));
/// ```
#[macro_export]
macro_rules! pad_start {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::pad_start($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pad_start($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pad_start($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pad_start($a, $b, $c)
    };
}

use crate::lib::{json, Value};
use super::pad::make_padding;

// internal `&str`/primitive worker for [pad_end()] / [pad_end_x()]
fn x_pad_end_x(s: &str, length: usize, chars: &str) -> String {
    let len = s.chars().count();
    if length <= len || chars.is_empty() {
        return s.into();
    }
    let mut out = s.to_owned();
    out.push_str(&make_padding(length - len, chars));
    out
}

/// See lodash [padEnd](https://lodash.com/docs/#padEnd)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pad_end;
/// # use serde_json::json;
/// assert_eq!(pad_end(json!("abc"), 6, "_-"), json!("abc_-_"));
/// ```
pub fn pad_end<A: Into<Value>>(v: A, length: usize, chars: &str) -> Value {
    let v = v.into();
    json!(x_pad_end_x(&crate::to_string_x(v), length, chars))
}

/// Based on [pad_end()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   pad_end!(json!("abc"), 6),
///   json!("abc   ")
/// );
/// assert_eq!(
///   pad_end!(json!("abc"), 6, "_-"),
///   json!("abc_-_")
/// );
/// assert_eq!(
///   pad_end!(json!("abc"), 3),
///   json!("abc")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pad_end!(), json!(""));
/// assert_eq!(pad_end!(json!("abc")), json!("abc"));
/// assert_eq!(pad_end!(json!(null), 2), json!("  "));
/// ```
#[macro_export]
macro_rules! pad_end {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::pad_end($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pad_end($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pad_end($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pad_end($a, $b, $c)
    };
}

/// `_x` helper for [pad_end()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pad_end_x;
/// # use serde_json::json;
/// assert_eq!(pad_end_x(json!("abc"), 6, "_-"), "abc_-_".to_owned());
/// ```
pub fn pad_end_x<A: Into<Value>>(v: A, length: usize, chars: &str) -> String {
    let v = v.into();
    x_pad_end_x(&crate::to_string_x(v), length, chars)
}

/// Based on [pad_end_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pad_end_x!(json!("abc"), 6, "_-"), "abc_-_".to_owned());
/// ```
macro_rules! pad_end_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::pad_end_x($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pad_end_x($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pad_end_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pad_end_x($a, $b, $c)
    };
}

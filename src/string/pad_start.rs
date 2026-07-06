use crate::lib::{json, Value};
use super::pad::make_padding;

/// `x_`/`_x` helper for [pad_start()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_pad_start_x;
/// # use serde_json::json;
/// assert_eq!(x_pad_start_x("abc", 6, "_-"), "_-_abc".to_owned());
/// ```
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
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pad_start;
/// # use serde_json::json;
/// assert_eq!(pad_start(json!("abc"), 6, "_-"), json!("_-_abc"));
/// ```
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
/// Additional cases:
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
        $crate::lib::json!("")
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

/// `x_` helper for [pad_start()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_pad_start;
/// # use serde_json::json;
/// assert_eq!(x_pad_start("abc", 6, "_-"), json!("_-_abc"));
/// ```
pub fn x_pad_start(s: &str, length: usize, chars: &str) -> Value {
    json!(x_pad_start_x(s, length, chars))
}
/// `_x` helper for [pad_start()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pad_start_x;
/// # use serde_json::json;
/// assert_eq!(pad_start_x(json!("abc"), 6, "_-"), "_-_abc".to_owned());
/// ```
pub fn pad_start_x(v: Value, length: usize, chars: &str) -> String {
    x_pad_start_x(&crate::to_string_x(v), length, chars)
}

/// Based on [x_pad_start_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_pad_start_x!("abc", 6, "_-"), "_-_abc".to_owned());
/// ```
macro_rules! x_pad_start_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_pad_start_x($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_pad_start_x($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_pad_start_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_pad_start_x($a, $b, $c)
    };
}
/// Based on [x_pad_start()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_pad_start!("abc", 6, "_-"), json!("_-_abc"));
/// ```
macro_rules! x_pad_start {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_pad_start($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_pad_start($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_pad_start($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_pad_start($a, $b, $c)
    };
}
/// Based on [pad_start_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pad_start_x!(json!("abc"), 6, "_-"), "_-_abc".to_owned());
/// ```
macro_rules! pad_start_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::pad_start_x($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pad_start_x($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pad_start_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pad_start_x($a, $b, $c)
    };
}

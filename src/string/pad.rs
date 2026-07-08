use crate::lib::{Value, json};

pub(crate) fn make_padding(n: usize, chars: &str) -> String {
    chars.chars().cycle().take(n).collect()
}

// internal `&str`/primitive worker for [pad()] / [pad_x()]
fn x_pad_x(s: &str, length: usize, chars: &str) -> String {
    let len = s.chars().count();
    if length <= len || chars.is_empty() {
        return s.into();
    }
    let total = length - len;
    let start = total / 2;
    let mut out = make_padding(start, chars);
    out.push_str(s);
    out.push_str(&make_padding(total - start, chars));
    out
}

/// Fn form of [pad!](crate::pad!); see it for the full docs
///
/// `_x` forms: [pad_x!](crate::pad_x!), [pad_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pad;
/// # use serde_json::json;
/// assert_eq!(pad(json!("abc"), 8, "_-"), json!("_-abc_-_"));
/// ```
pub fn pad<A: Into<Value>>(v: A, length: usize, chars: &str) -> Value {
    let v = v.into();
    json!(x_pad_x(&crate::to_string_x(v), length, chars))
}

/// See lodash [pad](https://lodash.com/docs/#pad)
///
/// Fn form: [pad()] | `_x` forms: [pad_x!](crate::pad_x!), [pad_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   pad!(json!("abc"), 8),
///   json!("  abc   ")
/// );
/// assert_eq!(
///   pad!(json!("abc"), 8, "_-"),
///   json!("_-abc_-_")
/// );
/// assert_eq!(
///   pad!(json!("abc"), 3),
///   json!("abc")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pad!(), json!(""));
/// assert_eq!(pad!(json!("abc")), json!("abc"));
/// assert_eq!(pad!(json!(null), 4), json!("    "));
/// assert_eq!(pad!(json!("abc"), 8, ""), json!("abc"));
/// ```
#[macro_export]
macro_rules! pad {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::pad($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pad($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pad($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pad($a, $b, $c)
    };
}

/// `_x` helper for [pad!](crate::pad!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [pad_x!](crate::pad_x!) | `Value` forms: [pad!](crate::pad!), [pad()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pad_x;
/// # use serde_json::json;
/// assert_eq!(pad_x(json!("abc"), 8, "_-"), "_-abc_-_".to_owned());
/// ```
pub fn pad_x<A: Into<Value>>(v: A, length: usize, chars: &str) -> String {
    let v = v.into();
    x_pad_x(&crate::to_string_x(v), length, chars)
}

/// `_x` helper for [pad!](crate::pad!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [pad_x()] | `Value` forms: [pad!](crate::pad!), [pad()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pad_x!(json!("abc"), 8, "_-"), "_-abc_-_".to_owned());
/// ```
#[macro_export]
macro_rules! pad_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::pad_x($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pad_x($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pad_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pad_x($a, $b, $c)
    };
}

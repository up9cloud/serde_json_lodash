use crate::lib::{Value, json};

// internal `&str`/primitive worker for [trim_start()] / [trim_start_x()]
fn x_trim_start_x(s: &str, chars: &str) -> String {
    if chars.is_empty() {
        return s.into();
    }
    s.trim_start_matches(|c| chars.contains(c)).into()
}

/// Fn form of [trim_start!](crate::trim_start!); see it for the full docs
///
/// `_x` forms: [trim_start_x!](crate::trim_start_x!), [trim_start_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_start;
/// # use serde_json::json;
/// assert_eq!(trim_start(json!("-_-abc-_-"), "_-"), json!("abc-_-"));
/// ```
pub fn trim_start<A: Into<Value>>(v: A, chars: &str) -> Value {
    let v = v.into();
    json!(x_trim_start_x(&crate::to_string_x(v), chars))
}

/// See lodash [trimStart](https://lodash.com/docs/#trimStart)
///
/// Fn form: [trim_start()] | `_x` forms: [trim_start_x!](crate::trim_start_x!), [trim_start_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   trim_start!(json!("  abc  ")),
///   json!("abc  ")
/// );
/// assert_eq!(
///   trim_start!(json!("-_-abc-_-"), "_-"),
///   json!("abc-_-")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim_start!(), json!(""));
/// assert_eq!(trim_start!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! trim_start {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::trim_start($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::trim_start($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::trim_start($a, $b)
    };
}

/// `_x` helper for [trim_start!](crate::trim_start!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [trim_start_x!](crate::trim_start_x!) | `Value` forms: [trim_start!](crate::trim_start!), [trim_start()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_start_x;
/// # use serde_json::json;
/// assert_eq!(trim_start_x(json!("-_-abc-_-"), "_-"), "abc-_-".to_owned());
/// ```
pub fn trim_start_x<A: Into<Value>>(v: A, chars: &str) -> String {
    let v = v.into();
    x_trim_start_x(&crate::to_string_x(v), chars)
}

/// `_x` helper for [trim_start!](crate::trim_start!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [trim_start_x()] | `Value` forms: [trim_start!](crate::trim_start!), [trim_start()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim_start_x!(json!("-_-abc-_-"), "_-"), "abc-_-".to_owned());
/// ```
#[macro_export]
macro_rules! trim_start_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::trim_start_x($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::trim_start_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::trim_start_x($a, $b)
    };
}

use crate::lib::{Value, json};

// internal `&str`/primitive worker for [trim_end()] / [trim_end_x()]
fn x_trim_end_x(s: &str, chars: &str) -> String {
    if chars.is_empty() {
        return s.into();
    }
    s.trim_end_matches(|c| chars.contains(c)).into()
}

/// Fn form of [trim_end!](crate::trim_end!); see it for the full docs
///
/// `_x` forms: [trim_end_x!](crate::trim_end_x!), [trim_end_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_end;
/// # use serde_json::json;
/// assert_eq!(trim_end(json!("-_-abc-_-"), "_-"), json!("-_-abc"));
/// ```
pub fn trim_end<A: Into<Value>>(v: A, chars: &str) -> Value {
    let v = v.into();
    json!(x_trim_end_x(&crate::to_string_x(v), chars))
}

/// See lodash [trimEnd](https://lodash.com/docs/#trimEnd)
///
/// Fn form: [trim_end()] | `_x` forms: [trim_end_x!](crate::trim_end_x!), [trim_end_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   trim_end!(json!("  abc  ")),
///   json!("  abc")
/// );
/// assert_eq!(
///   trim_end!(json!("-_-abc-_-"), "_-"),
///   json!("-_-abc")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim_end!(), json!(""));
/// assert_eq!(trim_end!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! trim_end {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::trim_end($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::trim_end($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::trim_end($a, $b)
    };
}

/// `_x` helper for [trim_end!](crate::trim_end!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [trim_end_x!](crate::trim_end_x!) | `Value` forms: [trim_end!](crate::trim_end!), [trim_end()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_end_x;
/// # use serde_json::json;
/// assert_eq!(trim_end_x(json!("-_-abc-_-"), "_-"), "-_-abc".to_owned());
/// ```
pub fn trim_end_x<A: Into<Value>>(v: A, chars: &str) -> String {
    let v = v.into();
    x_trim_end_x(&crate::to_string_x(v), chars)
}

/// `_x` helper for [trim_end!](crate::trim_end!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [trim_end_x()] | `Value` forms: [trim_end!](crate::trim_end!), [trim_end()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim_end_x!(json!("-_-abc-_-"), "_-"), "-_-abc".to_owned());
/// ```
#[macro_export]
macro_rules! trim_end_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::trim_end_x($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::trim_end_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::trim_end_x($a, $b)
    };
}

use crate::lib::{json, Value};

// internal `&str`/primitive worker for [trim()] / [trim_x()]
fn x_trim_x(s: &str, chars: &str) -> String {
    if chars.is_empty() {
        return s.into();
    }
    s.trim_matches(|c| chars.contains(c)).into()
}

/// See lodash [trim](https://lodash.com/docs/#trim)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim;
/// # use serde_json::json;
/// assert_eq!(trim(json!("-_-abc-_-"), "_-"), json!("abc"));
/// ```
pub fn trim<A: Into<Value>>(v: A, chars: &str) -> Value {
    let v = v.into();
    json!(x_trim_x(&crate::to_string_x(v), chars))
}

/// Based on [trim()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   trim!(json!("  abc  ")),
///   json!("abc")
/// );
/// assert_eq!(
///   trim!(json!("-_-abc-_-"), "_-"),
///   json!("abc")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim!(), json!(""));
/// assert_eq!(trim!(json!(null)), json!(""));
/// assert_eq!(trim!(json!("  abc  "), ""), json!("  abc  "));
/// ```
#[macro_export]
macro_rules! trim {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::trim($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::trim($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::trim($a, $b)
    };
}

/// `_x` helper for [trim()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_x;
/// # use serde_json::json;
/// assert_eq!(trim_x(json!("-_-abc-_-"), "_-"), "abc".to_owned());
/// ```
pub fn trim_x<A: Into<Value>>(v: A, chars: &str) -> String {
    let v = v.into();
    x_trim_x(&crate::to_string_x(v), chars)
}

/// Based on [trim_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim_x!(json!("-_-abc-_-"), "_-"), "abc".to_owned());
/// ```
macro_rules! trim_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::trim_x($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::trim_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::trim_x($a, $b)
    };
}

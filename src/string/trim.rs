use crate::lib::{json, Value};

/// `x_`/`_x` helper for [trim()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_trim_x;
/// # use serde_json::json;
/// assert_eq!(x_trim_x("-_-abc-_-", "_-"), "abc".to_owned());
/// ```
pub fn x_trim_x(s: &str, chars: &str) -> String {
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
pub fn trim(v: Value, chars: &str) -> Value {
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

/// `x_` helper for [trim()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_trim;
/// # use serde_json::json;
/// assert_eq!(x_trim("-_-abc-_-", "_-"), json!("abc"));
/// ```
pub fn x_trim(s: &str, chars: &str) -> Value {
    json!(x_trim_x(s, chars))
}
/// `_x` helper for [trim()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_x;
/// # use serde_json::json;
/// assert_eq!(trim_x(json!("-_-abc-_-"), "_-"), "abc".to_owned());
/// ```
pub fn trim_x(v: Value, chars: &str) -> String {
    x_trim_x(&crate::to_string_x(v), chars)
}

/// Based on [x_trim_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_trim_x!("-_-abc-_-", "_-"), "abc".to_owned());
/// ```
macro_rules! x_trim_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_trim_x($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_trim_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_trim_x($a, $b)
    };
}
/// Based on [x_trim()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_trim!("-_-abc-_-", "_-"), json!("abc"));
/// ```
macro_rules! x_trim {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_trim($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_trim($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_trim($a, $b)
    };
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

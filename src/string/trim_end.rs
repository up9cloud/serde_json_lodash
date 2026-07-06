use crate::lib::{json, Value};

/// `x_`/`_x` helper for [trim_end()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_trim_end_x;
/// # use serde_json::json;
/// assert_eq!(x_trim_end_x("-_-abc-_-", "_-"), "-_-abc".to_owned());
/// ```
pub fn x_trim_end_x(s: &str, chars: &str) -> String {
    if chars.is_empty() {
        return s.into();
    }
    s.trim_end_matches(|c| chars.contains(c)).into()
}
/// See lodash [trimEnd](https://lodash.com/docs/#trimEnd)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_end;
/// # use serde_json::json;
/// assert_eq!(trim_end(json!("-_-abc-_-"), "_-"), json!("-_-abc"));
/// ```
pub fn trim_end(v: Value, chars: &str) -> Value {
    json!(x_trim_end_x(&crate::to_string_x(v), chars))
}

/// Based on [trim_end()]
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

/// `x_` helper for [trim_end()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_trim_end;
/// # use serde_json::json;
/// assert_eq!(x_trim_end("-_-abc-_-", "_-"), json!("-_-abc"));
/// ```
pub fn x_trim_end(s: &str, chars: &str) -> Value {
    json!(x_trim_end_x(s, chars))
}
/// `_x` helper for [trim_end()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_end_x;
/// # use serde_json::json;
/// assert_eq!(trim_end_x(json!("-_-abc-_-"), "_-"), "-_-abc".to_owned());
/// ```
pub fn trim_end_x(v: Value, chars: &str) -> String {
    x_trim_end_x(&crate::to_string_x(v), chars)
}

/// Based on [x_trim_end_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_trim_end_x!("-_-abc-_-", "_-"), "-_-abc".to_owned());
/// ```
macro_rules! x_trim_end_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_trim_end_x($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_trim_end_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_trim_end_x($a, $b)
    };
}
/// Based on [x_trim_end()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_trim_end!("-_-abc-_-", "_-"), json!("-_-abc"));
/// ```
macro_rules! x_trim_end {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_trim_end($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_trim_end($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_trim_end($a, $b)
    };
}
/// Based on [trim_end_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim_end_x!(json!("-_-abc-_-"), "_-"), "-_-abc".to_owned());
/// ```
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

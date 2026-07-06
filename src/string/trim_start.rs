use crate::lib::{json, Value};

/// `x_`/`_x` helper for [trim_start()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_trim_start_x;
/// # use serde_json::json;
/// assert_eq!(x_trim_start_x("-_-abc-_-", "_-"), "abc-_-".to_owned());
/// ```
pub fn x_trim_start_x(s: &str, chars: &str) -> String {
    if chars.is_empty() {
        return s.into();
    }
    s.trim_start_matches(|c| chars.contains(c)).into()
}
/// See lodash [trimStart](https://lodash.com/docs/#trimStart)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_start;
/// # use serde_json::json;
/// assert_eq!(trim_start(json!("-_-abc-_-"), "_-"), json!("abc-_-"));
/// ```
pub fn trim_start(v: Value, chars: &str) -> Value {
    json!(x_trim_start_x(&crate::to_string_x(v), chars))
}

/// Based on [trim_start()]
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

/// `x_` helper for [trim_start()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_trim_start;
/// # use serde_json::json;
/// assert_eq!(x_trim_start("-_-abc-_-", "_-"), json!("abc-_-"));
/// ```
pub fn x_trim_start(s: &str, chars: &str) -> Value {
    json!(x_trim_start_x(s, chars))
}
/// `_x` helper for [trim_start()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::trim_start_x;
/// # use serde_json::json;
/// assert_eq!(trim_start_x(json!("-_-abc-_-"), "_-"), "abc-_-".to_owned());
/// ```
pub fn trim_start_x(v: Value, chars: &str) -> String {
    x_trim_start_x(&crate::to_string_x(v), chars)
}

/// Based on [x_trim_start_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_trim_start_x!("-_-abc-_-", "_-"), "abc-_-".to_owned());
/// ```
macro_rules! x_trim_start_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_trim_start_x($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_trim_start_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_trim_start_x($a, $b)
    };
}
/// Based on [x_trim_start()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_trim_start!("-_-abc-_-", "_-"), json!("abc-_-"));
/// ```
macro_rules! x_trim_start {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_trim_start($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_trim_start($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_trim_start($a, $b)
    };
}
/// Based on [trim_start_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim_start_x!(json!("-_-abc-_-"), "_-"), "abc-_-".to_owned());
/// ```
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

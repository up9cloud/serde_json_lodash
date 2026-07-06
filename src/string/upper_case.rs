use crate::lib::{json, Value};
use crate::internal;

/// `x_`/`_x` helper for [upper_case()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_upper_case_x;
/// # use serde_json::json;
/// assert_eq!(x_upper_case_x("--foo-bar"), "FOO BAR".to_owned());
/// ```
pub fn x_upper_case_x(s: &str) -> String {
    internal::compound_words(s)
        .iter()
        .map(|w| w.to_uppercase())
        .collect::<Vec<_>>()
        .join(" ")
}
/// `x_` helper for [upper_case()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_upper_case;
/// # use serde_json::json;
/// assert_eq!(x_upper_case("--foo-bar"), json!("FOO BAR"));
/// ```
pub fn x_upper_case(s: &str) -> Value {
    json!(x_upper_case_x(s))
}
/// `_x` helper for [upper_case()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::upper_case_x;
/// # use serde_json::json;
/// assert_eq!(upper_case_x(json!("--foo-bar")), "FOO BAR".to_owned());
/// ```
pub fn upper_case_x(v: Value) -> String {
    x_upper_case_x(&crate::to_string_x(v))
}
/// See lodash [upperCase](https://lodash.com/docs/#upperCase)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::upper_case;
/// # use serde_json::json;
/// assert_eq!(upper_case(json!("--foo-bar")), json!("FOO BAR"));
/// ```
pub fn upper_case(v: Value) -> Value {
    json!(upper_case_x(v))
}

/// Based on [upper_case()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   upper_case!(json!("--foo-bar")),
///   json!("FOO BAR")
/// );
/// assert_eq!(
///   upper_case!(json!("fooBar")),
///   json!("FOO BAR")
/// );
/// assert_eq!(
///   upper_case!(json!("__foo_bar__")),
///   json!("FOO BAR")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(upper_case!(), json!(""));
/// assert_eq!(upper_case!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! upper_case {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::upper_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::upper_case($a)
    };
}

/// Based on [x_upper_case_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_upper_case_x!("--foo-bar"), "FOO BAR".to_owned());
/// ```
macro_rules! x_upper_case_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_upper_case_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_upper_case_x($a)
    };
}
/// Based on [x_upper_case()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_upper_case!("--foo-bar"), json!("FOO BAR"));
/// ```
macro_rules! x_upper_case {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_upper_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_upper_case($a)
    };
}
/// Based on [upper_case_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(upper_case_x!(json!("--foo-bar")), "FOO BAR".to_owned());
/// ```
macro_rules! upper_case_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::upper_case_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::upper_case_x($a)
    };
}

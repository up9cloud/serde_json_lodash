use crate::lib::{json, Value};
use crate::internal;

// internal `&str`/primitive worker for [upper_case()] / [upper_case_x()]
fn x_upper_case_x(s: &str) -> String {
    internal::compound_words(s)
        .iter()
        .map(|w| w.to_uppercase())
        .collect::<Vec<_>>()
        .join(" ")
}

/// `_x` helper for [upper_case()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::upper_case_x;
/// # use serde_json::json;
/// assert_eq!(upper_case_x(json!("--foo-bar")), "FOO BAR".to_owned());
/// ```
pub fn upper_case_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
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
pub fn upper_case<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
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

use crate::lib::{Value, json};

use crate::internal;

// internal `&str`/primitive worker for [lower_case()] / [lower_case_x()]
fn x_lower_case_x(s: &str) -> String {
    internal::compound_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Fn form of [lower_case!](crate::lower_case!); see it for the full docs
///
/// `_x` forms: [lower_case_x!](crate::lower_case_x!), [lower_case_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::lower_case;
/// # use serde_json::json;
/// assert_eq!(lower_case(json!("--Foo-Bar--")), json!("foo bar"));
/// ```
pub fn lower_case<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    json!(lower_case_x(v))
}

/// See lodash [lowerCase](https://lodash.com/docs/#lowerCase)
///
/// Fn form: [lower_case()] | `_x` forms: [lower_case_x!](crate::lower_case_x!), [lower_case_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   lower_case!(json!("--Foo-Bar--")),
///   json!("foo bar")
/// );
/// assert_eq!(
///   lower_case!(json!("fooBar")),
///   json!("foo bar")
/// );
/// assert_eq!(
///   lower_case!(json!("__FOO_BAR__")),
///   json!("foo bar")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lower_case!(), json!(""));
/// assert_eq!(lower_case!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! lower_case {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::lower_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::lower_case($a)
    };
}

/// `_x` helper for [lower_case!](crate::lower_case!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [lower_case_x!](crate::lower_case_x!) | `Value` forms: [lower_case!](crate::lower_case!), [lower_case()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::lower_case_x;
/// # use serde_json::json;
/// assert_eq!(lower_case_x(json!("--Foo-Bar--")), "foo bar".to_owned());
/// ```
pub fn lower_case_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_lower_case_x(&crate::to_string_x(v))
}

/// `_x` helper for [lower_case!](crate::lower_case!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [lower_case_x()] | `Value` forms: [lower_case!](crate::lower_case!), [lower_case()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lower_case_x!(json!("--Foo-Bar--")), "foo bar".to_owned());
/// ```
#[macro_export]
macro_rules! lower_case_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::lower_case_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::lower_case_x($a)
    };
}

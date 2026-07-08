use crate::lib::{Value, json};

use crate::internal;

// internal `&str`/primitive worker for [kebab_case()] / [kebab_case_x()]
fn x_kebab_case_x(s: &str) -> String {
    internal::compound_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

/// Fn form of [kebab_case!](crate::kebab_case!); see it for the full docs
///
/// `_x` forms: [kebab_case_x!](crate::kebab_case_x!), [kebab_case_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::kebab_case;
/// # use serde_json::json;
/// assert_eq!(kebab_case(json!("Foo Bar")), json!("foo-bar"));
/// ```
pub fn kebab_case<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    json!(kebab_case_x(v))
}

/// See lodash [kebabCase](https://lodash.com/docs/#kebabCase)
///
/// Fn form: [kebab_case()] | `_x` forms: [kebab_case_x!](crate::kebab_case_x!), [kebab_case_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   kebab_case!(json!("Foo Bar")),
///   json!("foo-bar")
/// );
/// assert_eq!(
///   kebab_case!(json!("fooBar")),
///   json!("foo-bar")
/// );
/// assert_eq!(
///   kebab_case!(json!("__FOO_BAR__")),
///   json!("foo-bar")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(kebab_case!(), json!(""));
/// assert_eq!(kebab_case!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! kebab_case {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::kebab_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::kebab_case($a)
    };
}

/// `_x` helper for [kebab_case!](crate::kebab_case!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [kebab_case_x!](crate::kebab_case_x!) | `Value` forms: [kebab_case!](crate::kebab_case!), [kebab_case()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::kebab_case_x;
/// # use serde_json::json;
/// assert_eq!(kebab_case_x(json!("Foo Bar")), "foo-bar".to_owned());
/// ```
pub fn kebab_case_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_kebab_case_x(&crate::to_string_x(v))
}

/// `_x` helper for [kebab_case!](crate::kebab_case!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [kebab_case_x()] | `Value` forms: [kebab_case!](crate::kebab_case!), [kebab_case()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(kebab_case_x!(json!("Foo Bar")), "foo-bar".to_owned());
/// ```
#[macro_export]
macro_rules! kebab_case_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::kebab_case_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::kebab_case_x($a)
    };
}

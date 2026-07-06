use crate::lib::{json, Value};
use crate::internal;

// internal `&str`/primitive worker for [snake_case()] / [snake_case_x()]
fn x_snake_case_x(s: &str) -> String {
    internal::compound_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

/// `_x` helper for [snake_case()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::snake_case_x;
/// # use serde_json::json;
/// assert_eq!(snake_case_x(json!("Foo Bar")), "foo_bar".to_owned());
/// ```
pub fn snake_case_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_snake_case_x(&crate::to_string_x(v))
}

/// See lodash [snakeCase](https://lodash.com/docs/#snakeCase)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::snake_case;
/// # use serde_json::json;
/// assert_eq!(snake_case(json!("Foo Bar")), json!("foo_bar"));
/// ```
pub fn snake_case<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    json!(snake_case_x(v))
}

/// Based on [snake_case()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   snake_case!(json!("Foo Bar")),
///   json!("foo_bar")
/// );
/// assert_eq!(
///   snake_case!(json!("fooBar")),
///   json!("foo_bar")
/// );
/// assert_eq!(
///   snake_case!(json!("--FOO-BAR--")),
///   json!("foo_bar")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(snake_case!(), json!(""));
/// assert_eq!(snake_case!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! snake_case {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::snake_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::snake_case($a)
    };
}

/// Based on [snake_case_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(snake_case_x!(json!("Foo Bar")), "foo_bar".to_owned());
/// ```
macro_rules! snake_case_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::snake_case_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::snake_case_x($a)
    };
}

use crate::lib::{json, Value};
use crate::internal;

/// `x_`/`_x` helper for [lower_case()]: takes a primitive argument and returns a primitive value.
pub fn x_lower_case_x(s: &str) -> String {
    internal::compound_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}
/// `x_` helper for [lower_case()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_lower_case(s: &str) -> Value {
    json!(x_lower_case_x(s))
}
/// `_x` helper for [lower_case()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn lower_case_x(v: Value) -> String {
    x_lower_case_x(&crate::to_string_x(v))
}
/// See lodash [lowerCase](https://lodash.com/docs/#lowerCase)
pub fn lower_case(v: Value) -> Value {
    json!(lower_case_x(v))
}

/// Based on [lower_case()]
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
/// More examples:
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
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::lower_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::lower_case($a)
    };
}

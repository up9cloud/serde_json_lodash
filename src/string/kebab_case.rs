use crate::lib::{json, Value};
use crate::internal;

/// `x_`/`_x` helper for [kebab_case()]: takes a primitive argument and returns a primitive value.
pub fn x_kebab_case_x(s: &str) -> String {
    internal::compound_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}
/// `x_` helper for [kebab_case()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_kebab_case(s: &str) -> Value {
    json!(x_kebab_case_x(s))
}
/// `_x` helper for [kebab_case()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn kebab_case_x(v: Value) -> String {
    x_kebab_case_x(&crate::to_string_x(v))
}
/// See lodash [kebabCase](https://lodash.com/docs/#kebabCase)
pub fn kebab_case(v: Value) -> Value {
    json!(kebab_case_x(v))
}

/// Based on [kebab_case()]
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
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(kebab_case!(), json!(""));
/// assert_eq!(kebab_case!(json!(null)), json!(""));
/// assert_eq!(serde_json_lodash::x_kebab_case_x("HTMLParser"), "html-parser".to_owned());
/// ```
#[macro_export]
macro_rules! kebab_case {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::kebab_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::kebab_case($a)
    };
}

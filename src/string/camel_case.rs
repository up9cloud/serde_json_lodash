use crate::lib::{json, Value};
use crate::internal;

/// `x_`/`_x` helper for [camel_case()]: takes a primitive argument and returns a primitive value.
pub fn x_camel_case_x(s: &str) -> String {
    let mut out = String::new();
    for (i, w) in internal::compound_words(s).iter().enumerate() {
        if i == 0 {
            out.push_str(&w.to_lowercase());
        } else {
            out.push_str(&internal::capitalize_word(w));
        }
    }
    out
}
/// `x_` helper for [camel_case()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_camel_case(s: &str) -> Value {
    json!(x_camel_case_x(s))
}
/// `_x` helper for [camel_case()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn camel_case_x(v: Value) -> String {
    x_camel_case_x(&crate::to_string_x(v))
}
/// See lodash [camelCase](https://lodash.com/docs/#camelCase)
pub fn camel_case(v: Value) -> Value {
    json!(camel_case_x(v))
}

/// Based on [camel_case()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   camel_case!(json!("Foo Bar")),
///   json!("fooBar")
/// );
/// assert_eq!(
///   camel_case!(json!("--foo-bar--")),
///   json!("fooBar")
/// );
/// assert_eq!(
///   camel_case!(json!("__FOO_BAR__")),
///   json!("fooBar")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(camel_case!(), json!(""));
/// assert_eq!(camel_case!(json!(null)), json!(""));
/// assert_eq!(camel_case!(json!("foo2bar")), json!("foo2Bar"));
/// assert_eq!(x_camel_case!("déjà vu"), json!("dejaVu"));
/// assert_eq!(x_camel_case_x!("Foo Bar"), "fooBar".to_owned());
/// assert_eq!(camel_case_x!(json!("Foo Bar")), "fooBar".to_owned());
/// ```
#[macro_export]
macro_rules! camel_case {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::camel_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::camel_case($a)
    };
}
/// Based on [x_camel_case()]
#[macro_export]
macro_rules! x_camel_case {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_camel_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_camel_case($a)
    };
}
/// Based on [camel_case_x()]
#[macro_export]
macro_rules! camel_case_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::camel_case_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::camel_case_x($a)
    };
}
/// Based on [x_camel_case_x()]
#[macro_export]
macro_rules! x_camel_case_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_camel_case_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_camel_case_x($a)
    };
}

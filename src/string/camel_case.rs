use crate::lib::{json, Value};
use crate::internal;

// internal `&str`/primitive worker for [camel_case()] / [camel_case_x()]
fn x_camel_case_x(s: &str) -> String {
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

/// `_x` helper for [camel_case()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::camel_case_x;
/// # use serde_json::json;
/// assert_eq!(camel_case_x(json!("Foo Bar")), "fooBar".to_owned());
/// ```
pub fn camel_case_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_camel_case_x(&crate::to_string_x(v))
}

/// See lodash [camelCase](https://lodash.com/docs/#camelCase)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::camel_case;
/// # use serde_json::json;
/// assert_eq!(camel_case(json!("Foo Bar")), json!("fooBar"));
/// ```
pub fn camel_case<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
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
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(camel_case!(), json!(""));
/// assert_eq!(camel_case!(json!(null)), json!(""));
/// assert_eq!(camel_case!(json!("foo2bar")), json!("foo2Bar"));
/// assert_eq!(camel_case_x!(json!("Foo Bar")), "fooBar".to_owned());
/// ```
#[macro_export]
macro_rules! camel_case {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::camel_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::camel_case($a)
    };
}

/// Based on [camel_case_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(camel_case_x!(json!("Foo Bar")), "fooBar".to_owned());
/// ```
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

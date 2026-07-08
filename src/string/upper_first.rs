use crate::lib::{Value, json};

use crate::internal;

// internal `&str`/primitive worker for [upper_first()] / [upper_first_x()]
fn x_upper_first_x(s: &str) -> String {
    internal::upper_first_word(s)
}

/// Fn form of [upper_first!](crate::upper_first!); see it for the full docs
///
/// `_x` forms: [upper_first_x!](crate::upper_first_x!), [upper_first_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::upper_first;
/// # use serde_json::json;
/// assert_eq!(upper_first(json!("fred")), json!("Fred"));
/// ```
pub fn upper_first<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    json!(upper_first_x(v))
}

/// See lodash [upperFirst](https://lodash.com/docs/#upperFirst)
///
/// Fn form: [upper_first()] | `_x` forms: [upper_first_x!](crate::upper_first_x!), [upper_first_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   upper_first!(json!("fred")),
///   json!("Fred")
/// );
/// assert_eq!(
///   upper_first!(json!("FRED")),
///   json!("FRED")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(upper_first!(), json!(""));
/// assert_eq!(upper_first!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! upper_first {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::upper_first($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::upper_first($a)
    };
}

/// `_x` helper for [upper_first!](crate::upper_first!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [upper_first_x!](crate::upper_first_x!) | `Value` forms: [upper_first!](crate::upper_first!), [upper_first()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::upper_first_x;
/// # use serde_json::json;
/// assert_eq!(upper_first_x(json!("fred")), "Fred".to_owned());
/// ```
pub fn upper_first_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_upper_first_x(&crate::to_string_x(v))
}

/// `_x` helper for [upper_first!](crate::upper_first!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [upper_first_x()] | `Value` forms: [upper_first!](crate::upper_first!), [upper_first()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(upper_first_x!(json!("fred")), "Fred".to_owned());
/// ```
#[macro_export]
macro_rules! upper_first_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::upper_first_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::upper_first_x($a)
    };
}

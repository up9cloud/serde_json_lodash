use std::sync::Mutex;

use crate::lib::{Value, json};

lazy_static::lazy_static! {
    static ref COUNT: Mutex<usize> = Mutex::new(0);
}

/// Fn form of [unique_id!](crate::unique_id!); see it for the full docs
///
/// `_x` forms: [unique_id_x!](crate::unique_id_x!), [unique_id_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::unique_id;
/// # use serde_json::json;
/// assert_eq!(unique_id("contact_"), json!("contact_1"));
/// ```
pub fn unique_id<A: Into<Value>>(prefix: A) -> Value {
    json!(unique_id_x(prefix))
}

/// See lodash [uniqueId](https://lodash.com/docs/#uniqueId)
///
/// Accepts anything convertible into a `Value` — a `&str`/`String` primitive or a `json!` value.
///
/// Fn form: [unique_id()] | `_x` forms: [unique_id_x!](crate::unique_id_x!), [unique_id_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   unique_id!("contact_"),
///   json!("contact_1")
/// );
/// assert_eq!(
///   unique_id!(),
///   json!("2")
/// );
/// // a `json!` prefix works too
/// assert_eq!(unique_id!(json!("contact_")), json!("contact_3"));
/// ```
#[macro_export]
macro_rules! unique_id {
    () => {
        $crate::unique_id("")
    };
    ($a:expr $(,)*) => {
        $crate::unique_id($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::unique_id($a)
    };
}

/// `_x` helper for [unique_id!](crate::unique_id!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Accepts anything convertible into a `Value` — a `&str`/`String` primitive or a `json!` value.
///
/// Macro form: [unique_id_x!](crate::unique_id_x!) | `Value` forms: [unique_id!](crate::unique_id!), [unique_id()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::unique_id_x;
/// # use serde_json::json;
/// assert_eq!(unique_id_x("contact_"), "contact_1".to_owned());
/// ```
pub fn unique_id_x<A: Into<Value>>(prefix: A) -> String {
    let prefix = crate::to_string_x(prefix);
    let mut c = COUNT.lock().unwrap();
    *c += 1;
    format!("{}{}", prefix, c)
}

/// `_x` helper for [unique_id!](crate::unique_id!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [unique_id_x()] | `Value` forms: [unique_id!](crate::unique_id!), [unique_id()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(unique_id_x!("contact_"), "contact_1".to_owned());
/// ```
#[macro_export]
macro_rules! unique_id_x {
    () => {
        $crate::unique_id_x("")
    };
    ($a:expr $(,)*) => {
        $crate::unique_id_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::unique_id_x($a)
    };
}

use std::sync::Mutex;
use crate::lib::{json, Value};
lazy_static::lazy_static! {
    static ref COUNT: Mutex<usize> = Mutex::new(0);
}

/// `_x` helper for [unique_id()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Accepts anything convertible into a `Value` — a `&str`/`String` primitive or a `json!` value.
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
/// See lodash [uniqueId](https://lodash.com/docs/#uniqueId)
///
/// Accepts anything convertible into a `Value` — a `&str`/`String` primitive or a `json!` value.
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

/// Based on [unique_id_x()]
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
/// Based on [unique_id()]
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

use std::sync::Mutex;
use crate::lib::{json, Value};
lazy_static::lazy_static! {
    static ref COUNT: Mutex<usize> = Mutex::new(0);
}
///
pub fn x_unique_id_x(prefix: &str) -> String {
    let mut c = COUNT.lock().unwrap();
    *c += 1;
    format!("{}{}", prefix, c)
}
///
pub fn x_unique_id(prefix: &str) -> Value {
    json!(x_unique_id_x(prefix))
}
///
pub fn unique_id_x(prefix: &str) -> String {
    x_unique_id_x(prefix)
}
/// See lodash [uniqueId](https://lodash.com/docs/#uniqueId)
pub fn unique_id(prefix: &str) -> Value {
    x_unique_id(prefix)
}

/// Based on [x_unique_id_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   x_unique_id_x!("contact_"),
///   "contact_1".to_owned()
/// );
/// assert_eq!(
///   x_unique_id_x!(),
///   "2".to_owned()
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// ```
#[macro_export]
macro_rules! x_unique_id_x {
    () => {
        $crate::x_unique_id_x("")
    };
    ($a:expr $(,)*) => {
        $crate::x_unique_id_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_unique_id_x($a)
    };
}
/// Based on [x_unique_id()]
#[macro_export]
macro_rules! x_unique_id {
    () => {
        $crate::x_unique_id("")
    };
    ($a:expr $(,)*) => {
        $crate::x_unique_id($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_unique_id($a)
    };
}
/// Based on [unique_id_x()]
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
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
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

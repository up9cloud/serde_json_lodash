use crate::lib::{json, Value};
use crate::internal;

/// `x_`/`_x` helper for [upper_first()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_upper_first_x;
/// # use serde_json::json;
/// assert_eq!(x_upper_first_x("fred"), "Fred".to_owned());
/// ```
pub fn x_upper_first_x(s: &str) -> String {
    internal::upper_first_word(s)
}
/// `x_` helper for [upper_first()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_upper_first;
/// # use serde_json::json;
/// assert_eq!(x_upper_first("fred"), json!("Fred"));
/// ```
pub fn x_upper_first(s: &str) -> Value {
    json!(x_upper_first_x(s))
}
/// `_x` helper for [upper_first()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::upper_first_x;
/// # use serde_json::json;
/// assert_eq!(upper_first_x(json!("fred")), "Fred".to_owned());
/// ```
pub fn upper_first_x(v: Value) -> String {
    x_upper_first_x(&crate::to_string_x(v))
}
/// See lodash [upperFirst](https://lodash.com/docs/#upperFirst)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::upper_first;
/// # use serde_json::json;
/// assert_eq!(upper_first(json!("fred")), json!("Fred"));
/// ```
pub fn upper_first(v: Value) -> Value {
    json!(upper_first_x(v))
}

/// Based on [upper_first()]
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

/// Based on [x_upper_first_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_upper_first_x!("fred"), "Fred".to_owned());
/// ```
macro_rules! x_upper_first_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_upper_first_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_upper_first_x($a)
    };
}
/// Based on [x_upper_first()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_upper_first!("fred"), json!("Fred"));
/// ```
macro_rules! x_upper_first {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_upper_first($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_upper_first($a)
    };
}
/// Based on [upper_first_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(upper_first_x!(json!("fred")), "Fred".to_owned());
/// ```
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

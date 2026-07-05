use crate::lib::{json, Value};
use crate::internal;

/// `x_`/`_x` helper for [upper_first()]: takes a primitive argument and returns a primitive value.
pub fn x_upper_first_x(s: &str) -> String {
    internal::upper_first_word(s)
}
/// `x_` helper for [upper_first()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_upper_first(s: &str) -> Value {
    json!(x_upper_first_x(s))
}
/// `_x` helper for [upper_first()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn upper_first_x(v: Value) -> String {
    x_upper_first_x(&crate::to_string_x(v))
}
/// See lodash [upperFirst](https://lodash.com/docs/#upperFirst)
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
/// More examples:
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
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::upper_first($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::upper_first($a)
    };
}

use crate::lib::{json, Value};

/// `x_`/`_x` helper for [to_upper()]: takes a primitive argument and returns a primitive value.
pub fn x_to_upper_x(s: &str) -> String {
    s.to_uppercase()
}
/// `x_` helper for [to_upper()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_to_upper(s: &str) -> Value {
    json!(x_to_upper_x(s))
}
/// `_x` helper for [to_upper()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn to_upper_x(v: Value) -> String {
    x_to_upper_x(&crate::to_string_x(v))
}
/// See lodash [toUpper](https://lodash.com/docs/#toUpper)
pub fn to_upper(v: Value) -> Value {
    json!(to_upper_x(v))
}

/// Based on [to_upper()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   to_upper!(json!("--foo-bar--")),
///   json!("--FOO-BAR--")
/// );
/// assert_eq!(
///   to_upper!(json!("fooBar")),
///   json!("FOOBAR")
/// );
/// assert_eq!(
///   to_upper!(json!("__foo_bar__")),
///   json!("__FOO_BAR__")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_upper!(), json!(""));
/// assert_eq!(to_upper!(json!(null)), json!(""));
/// assert_eq!(to_upper!(json!([1, 2, 3])), json!("1,2,3"));
/// ```
#[macro_export]
macro_rules! to_upper {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::to_upper($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_upper($a)
    };
}

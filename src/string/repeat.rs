use crate::lib::{json, Value};

/// `x_`/`_x` helper for [repeat()]: takes a primitive argument and returns a primitive value.
pub fn x_repeat_x(s: &str, n: usize) -> String {
    s.repeat(n)
}
/// See lodash [repeat](https://lodash.com/docs/#repeat)
pub fn repeat(v: Value, n: usize) -> Value {
    json!(x_repeat_x(&crate::to_string_x(v), n))
}

/// Based on [repeat()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   repeat!(json!("*"), 3),
///   json!("***")
/// );
/// assert_eq!(
///   repeat!(json!("abc"), 2),
///   json!("abcabc")
/// );
/// assert_eq!(
///   repeat!(json!("abc"), 0),
///   json!("")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(repeat!(), json!(""));
/// assert_eq!(repeat!(json!("abc")), json!("abc"));
/// assert_eq!(repeat!(json!(null), 2), json!(""));
/// ```
#[macro_export]
macro_rules! repeat {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::repeat($a, 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::repeat($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::repeat($a, $b)
    };
}

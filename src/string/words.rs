use crate::lib::{json, Value};
use crate::internal;

/// `x_` helper for [words()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_words(s: &str) -> Value {
    json!(internal::words_vec(s))
}
/// See lodash [words](https://lodash.com/docs/#words)
///
/// *Note:* the `[pattern]` parameter is not supported, only the default
/// lodash word splitting logic is implemented
pub fn words(v: Value) -> Value {
    x_words(&crate::to_string_x(v))
}

/// Based on [words()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   words!(json!("fred, barney, & pebbles")),
///   json!(["fred", "barney", "pebbles"])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(words!(), json!([]));
/// assert_eq!(words!(json!(null)), json!([]));
/// assert_eq!(words!(json!("fred12barney")), json!(["fred", "12", "barney"]));
/// assert_eq!(words!(json!("FOOBar")), json!(["FOO", "Bar"]));
/// assert_eq!(words!(json!("don't, oh my")), json!(["don't", "oh", "my"]));
/// assert_eq!(x_words!("déjà vu"), json!(["déjà", "vu"]));
/// ```
#[macro_export]
macro_rules! words {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::words($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::words($a)
    };
}
/// Based on [x_words()]
#[macro_export]
macro_rules! x_words {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::x_words($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_words($a)
    };
}

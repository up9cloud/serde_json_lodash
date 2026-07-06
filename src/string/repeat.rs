use crate::lib::{json, Value};

// internal `&str`/primitive worker for [repeat()] / [repeat_x()]
fn x_repeat_x(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// See lodash [repeat](https://lodash.com/docs/#repeat)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::repeat;
/// # use serde_json::json;
/// assert_eq!(repeat(json!("*"), 3), json!("***"));
/// ```
pub fn repeat<A: Into<Value>>(v: A, n: usize) -> Value {
    let v = v.into();
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
/// Additional cases:
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
        $crate::lib::json!("")
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

/// `_x` helper for [repeat()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::repeat_x;
/// # use serde_json::json;
/// assert_eq!(repeat_x(json!("*"), 3), "***".to_owned());
/// ```
pub fn repeat_x<A: Into<Value>>(v: A, n: usize) -> String {
    let v = v.into();
    x_repeat_x(&crate::to_string_x(v), n)
}

/// Based on [repeat_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(repeat_x!(json!("*"), 3), "***".to_owned());
/// ```
macro_rules! repeat_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::repeat_x($a, 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::repeat_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::repeat_x($a, $b)
    };
}

use crate::lib::{Value, json};

/// Fn form of [take_while!](crate::take_while!); see it for the full docs
///
/// `_x` form: **not provided** — see [take_while_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::take_while;
/// # use serde_json::json;
/// assert_eq!(take_while(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() < 3), json!([1, 2]));
/// ```
pub fn take_while(array: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => Value::Array(vec.into_iter().take_while(predicate).collect()),
        _ => json!([]),
    }
}

/// See lodash [takeWhile](https://lodash.com/docs/#takeWhile)
///
/// Takes elements from the start while `predicate` returns `true`
///
/// Fn form: [take_while()] | `_x` form: **not provided** — see [take_while_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   take_while!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() < 3),
///   json!([1, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(take_while!(), json!([]));
/// assert_eq!(take_while!(json!([1, 2, 3])), json!([]));
/// assert_eq!(take_while!(json!([1, 2, 3]), |_| true), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! take_while {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::take_while($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::take_while($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [take_while!](crate::take_while!) and read the returned
/// `Value`.
///
/// Macro form: [take_while_x!](crate::take_while_x!)
pub fn take_while_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [take_while!](crate::take_while!) and read the returned
/// `Value`.
///
/// Fn form: [take_while_x()]
#[macro_export]
macro_rules! take_while_x {
    ($($t:tt)*) => {
        $crate::take_while_x()
    };
}

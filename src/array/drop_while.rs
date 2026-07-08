use crate::lib::{Value, json};

/// Fn form of [drop_while!](crate::drop_while!); see it for the full docs
///
/// `_x` form: **not provided** — see [drop_while_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::drop_while;
/// # use serde_json::json;
/// assert_eq!(drop_while(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() < 3), json!([3, 4]));
/// ```
pub fn drop_while(array: Value, predicate: fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => Value::Array(vec.into_iter().skip_while(predicate).collect()),
        _ => json!([]),
    }
}

/// See lodash [dropWhile](https://lodash.com/docs/#dropWhile)
///
/// Drops elements from the start while `predicate` returns `true`
///
/// Fn form: [drop_while()] | `_x` form: **not provided** — see [drop_while_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   drop_while!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() < 3),
///   json!([3, 4])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(drop_while!(), json!([]));
/// assert_eq!(drop_while!(json!([1, 2, 3])), json!([1, 2, 3]));
/// assert_eq!(drop_while!(json!([1, 2, 3]), |_| true), json!([]));
/// ```
#[macro_export]
macro_rules! drop_while {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::drop_while($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::drop_while($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [drop_while!](crate::drop_while!) and read the returned
/// `Value`.
///
/// Macro form: [drop_while_x!](crate::drop_while_x!)
pub fn drop_while_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [drop_while!](crate::drop_while!) and read the returned
/// `Value`.
///
/// Fn form: [drop_while_x()]
#[macro_export]
macro_rules! drop_while_x {
    ($($t:tt)*) => {
        $crate::drop_while_x()
    };
}

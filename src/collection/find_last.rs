use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [find_last!](crate::find_last!); see it for the full docs
///
/// `_x` form: **not provided** — see [find_last_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_last;
/// # use serde_json::json;
/// assert_eq!(find_last(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1), json!(3));
/// ```
pub fn find_last(collection: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    collection_values(collection)
        .into_iter()
        .rev()
        .find(predicate)
        .unwrap_or(Value::Null)
}

/// See lodash [findLast](https://lodash.com/docs/#findLast)
///
/// Like [find()](fn@crate::find) but iterates from the end
///
/// Fn form: [find_last()] | `_x` form: **not provided** — see [find_last_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   find_last!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1),
///   json!(3)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_last!(), json!(null));
/// assert_eq!(find_last!(json!([1, 2, 3]), |_| false), json!(null));
/// ```
#[macro_export]
macro_rules! find_last {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(null)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find_last($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::find_last($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [find_last!](crate::find_last!) and read the returned
/// `Value`.
///
/// Macro form: [find_last_x!](crate::find_last_x!)
pub fn find_last_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [find_last!](crate::find_last!) and read the returned
/// `Value`.
///
/// Fn form: [find_last_x()]
#[macro_export]
macro_rules! find_last_x {
    ($($t:tt)*) => {
        $crate::find_last_x()
    };
}

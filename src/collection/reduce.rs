use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [reduce!](crate::reduce!); see it for the full docs
///
/// `_x` form: **not provided** — see [reduce_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::reduce;
/// # use serde_json::json;
/// assert_eq!(reduce(json!([1, 2, 3]), |acc, n| json!(acc.as_i64().unwrap() + n.as_i64().unwrap()), json!(0)), json!(6));
/// ```
pub fn reduce(
    collection: Value,
    iteratee: impl Fn(Value, &Value) -> Value,
    accumulator: Value,
) -> Value {
    collection_values(collection)
        .iter()
        .fold(accumulator, iteratee)
}

/// See lodash [reduce](https://lodash.com/docs/#reduce)
///
/// `iteratee` receives `(accumulator, value)` and returns the next
/// accumulator
///
/// Fn form: [reduce()] | `_x` form: **not provided** — see [reduce_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   reduce!(json!([1, 2, 3]), |acc, n| json!(acc.as_i64().unwrap() + n.as_i64().unwrap()), json!(0)),
///   json!(6)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(reduce!(), json!(null));
/// assert_eq!(
///   reduce!(json!([1, 2]), |acc, n| json!(acc.as_i64().unwrap() + n.as_i64().unwrap()), json!(10)),
///   json!(13)
/// );
/// ```
#[macro_export]
macro_rules! reduce {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::reduce($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::reduce($a, $b, $c)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [reduce!](crate::reduce!) and read the returned
/// `Value`.
///
/// Macro form: [reduce_x!](crate::reduce_x!)
pub fn reduce_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [reduce!](crate::reduce!) and read the returned
/// `Value`.
///
/// Fn form: [reduce_x()]
#[macro_export]
macro_rules! reduce_x {
    ($($t:tt)*) => {
        $crate::reduce_x()
    };
}

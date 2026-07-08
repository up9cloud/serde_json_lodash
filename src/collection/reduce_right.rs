use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [reduce_right!](crate::reduce_right!); see it for the full docs
///
/// `_x` form: **not provided** — see [reduce_right_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::reduce_right;
/// # use serde_json::json;
/// assert_eq!(reduce_right(json!([[0, 1], [2, 3], [4, 5]]), |mut acc, n| {     let a = acc.as_array_mut().unwrap();     for x in n.as_array().unwrap() { a.push(x.clone()); }     acc   }, json!([])), json!([4, 5, 2, 3, 0, 1]));
/// ```
pub fn reduce_right(
    collection: Value,
    iteratee: fn(Value, &Value) -> Value,
    accumulator: Value,
) -> Value {
    collection_values(collection)
        .iter()
        .rev()
        .fold(accumulator, iteratee)
}

/// See lodash [reduceRight](https://lodash.com/docs/#reduceRight)
///
/// Like [reduce()](fn@crate::reduce) but iterates from the end
///
/// Fn form: [reduce_right()] | `_x` form: **not provided** — see [reduce_right_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   reduce_right!(json!([[0, 1], [2, 3], [4, 5]]), |mut acc, n| {
///     let a = acc.as_array_mut().unwrap();
///     for x in n.as_array().unwrap() { a.push(x.clone()); }
///     acc
///   }, json!([])),
///   json!([4, 5, 2, 3, 0, 1])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(reduce_right!(), json!(null));
/// ```
#[macro_export]
macro_rules! reduce_right {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::reduce_right($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::reduce_right($a, $b, $c)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [reduce_right!](crate::reduce_right!) and read the
/// returned `Value`.
///
/// Macro form: [reduce_right_x!](crate::reduce_right_x!)
pub fn reduce_right_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [reduce_right!](crate::reduce_right!) and read the
/// returned `Value`.
///
/// Fn form: [reduce_right_x()]
#[macro_export]
macro_rules! reduce_right_x {
    ($($t:tt)*) => {
        $crate::reduce_right_x()
    };
}

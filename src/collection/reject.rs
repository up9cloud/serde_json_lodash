use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [reject!](crate::reject!); see it for the full docs
///
/// `_x` form: **not provided** — see [reject_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::reject;
/// # use serde_json::json;
/// assert_eq!(reject(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1), json!([2, 4]));
/// ```
pub fn reject(collection: Value, predicate: fn(&Value) -> bool) -> Value {
    Value::Array(
        collection_values(&collection)
            .into_iter()
            .filter(|v| !predicate(v))
            .collect(),
    )
}

/// See lodash [reject](https://lodash.com/docs/#reject)
///
/// The opposite of [filter()](fn@crate::filter)
///
/// Fn form: [reject()] | `_x` form: **not provided** — see [reject_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   reject!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1),
///   json!([2, 4])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(reject!(), json!([]));
/// assert_eq!(reject!(json!([1, 2, 3])), json!([]));
/// ```
#[macro_export]
macro_rules! reject {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::reject($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::reject($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [reject!](crate::reject!) and read the returned
/// `Value`.
///
/// Macro form: [reject_x!](crate::reject_x!)
pub fn reject_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [reject!](crate::reject!) and read the returned
/// `Value`.
///
/// Fn form: [reject_x()]
#[macro_export]
macro_rules! reject_x {
    ($($t:tt)*) => {
        $crate::reject_x()
    };
}

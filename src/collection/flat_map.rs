use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [flat_map!](crate::flat_map!); see it for the full docs
///
/// `_x` form: **not provided** — see [flat_map_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::flat_map;
/// # use serde_json::json;
/// assert_eq!(flat_map(json!([1, 2]), |n| json!([n.clone(), n.clone()])), json!([1, 1, 2, 2]));
/// ```
pub fn flat_map(collection: Value, iteratee: fn(&Value) -> Value) -> Value {
    let mut out = vec![];
    for v in collection_values(collection) {
        match iteratee(&v) {
            Value::Array(inner) => out.extend(inner),
            other => out.push(other),
        }
    }
    Value::Array(out)
}

/// See lodash [flatMap](https://lodash.com/docs/#flatMap)
///
/// Maps each element with `iteratee`, then flattens the result one level
///
/// Fn form: [flat_map()] | `_x` form: **not provided** — see [flat_map_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   flat_map!(json!([1, 2]), |n| json!([n.clone(), n.clone()])),
///   json!([1, 1, 2, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(flat_map!(), json!([]));
/// assert_eq!(flat_map!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! flat_map {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::flat_map($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::flat_map($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::flat_map($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [flat_map!](crate::flat_map!) and read the returned
/// `Value`.
///
/// Macro form: [flat_map_x!](crate::flat_map_x!)
pub fn flat_map_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [flat_map!](crate::flat_map!) and read the returned
/// `Value`.
///
/// Fn form: [flat_map_x()]
#[macro_export]
macro_rules! flat_map_x {
    ($($t:tt)*) => {
        $crate::flat_map_x()
    };
}

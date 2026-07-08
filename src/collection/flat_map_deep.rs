use crate::lib::Value;

use crate::collection::collect::collection_values;

fn flatten_all(v: Value, out: &mut Vec<Value>) {
    match v {
        Value::Array(inner) => {
            for i in inner {
                flatten_all(i, out);
            }
        }
        other => out.push(other),
    }
}

/// Fn form of [flat_map_deep!](crate::flat_map_deep!); see it for the full docs
///
/// `_x` form: **not provided** — see [flat_map_deep_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::flat_map_deep;
/// # use serde_json::json;
/// assert_eq!(flat_map_deep(json!([1, 2]), |n| json!([[n.clone(), n.clone()]])), json!([1, 1, 2, 2]));
/// ```
pub fn flat_map_deep(collection: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    let mut out = vec![];
    for v in collection_values(collection) {
        flatten_all(iteratee(&v), &mut out);
    }
    Value::Array(out)
}

/// See lodash [flatMapDeep](https://lodash.com/docs/#flatMapDeep)
///
/// Maps each element with `iteratee`, then recursively flattens the result
///
/// Fn form: [flat_map_deep()] | `_x` form: **not provided** — see [flat_map_deep_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   flat_map_deep!(json!([1, 2]), |n| json!([[n.clone(), n.clone()]])),
///   json!([1, 1, 2, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(flat_map_deep!(), json!([]));
/// assert_eq!(flat_map_deep!(json!([[1], [[2]]])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! flat_map_deep {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::flat_map_deep($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::flat_map_deep($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::flat_map_deep($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [flat_map_deep!](crate::flat_map_deep!) and read the
/// returned `Value`.
///
/// Macro form: [flat_map_deep_x!](crate::flat_map_deep_x!)
pub fn flat_map_deep_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [flat_map_deep!](crate::flat_map_deep!) and read the
/// returned `Value`.
///
/// Fn form: [flat_map_deep_x()]
#[macro_export]
macro_rules! flat_map_deep_x {
    ($($t:tt)*) => {
        $crate::flat_map_deep_x()
    };
}

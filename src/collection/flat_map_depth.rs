use crate::lib::Value;

use crate::collection::collect::collection_values;

fn flatten_depth(v: Value, depth: isize, out: &mut Vec<Value>) {
    match v {
        Value::Array(inner) if depth > 0 => {
            for i in inner {
                flatten_depth(i, depth - 1, out);
            }
        }
        other => out.push(other),
    }
}

/// Fn form of [flat_map_depth!](crate::flat_map_depth!); see it for the full docs
///
/// `_x` form: **not provided** — see [flat_map_depth_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::flat_map_depth;
/// # use serde_json::json;
/// assert_eq!(flat_map_depth(json!([1, 2]), |n| json!([[n.clone(), n.clone()]]), 2), json!([1, 1, 2, 2]));
/// ```
pub fn flat_map_depth(
    collection: Value,
    iteratee: impl Fn(&Value) -> Value,
    depth: isize,
) -> Value {
    let mut out = vec![];
    for v in collection_values(collection) {
        // the iteratee result is flattened `depth` levels, mirroring lodash
        flatten_depth(iteratee(&v), depth, &mut out);
    }
    Value::Array(out)
}

/// See lodash [flatMapDepth](https://lodash.com/docs/#flatMapDepth)
///
/// Maps each element with `iteratee`, then flattens the result up to `depth`
/// levels
///
/// Fn form: [flat_map_depth()] | `_x` form: **not provided** — see [flat_map_depth_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   flat_map_depth!(json!([1, 2]), |n| json!([[n.clone(), n.clone()]]), 2),
///   json!([1, 1, 2, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(flat_map_depth!(), json!([]));
/// assert_eq!(flat_map_depth!(json!([[1], [2]]), |v| v.clone(), 1), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! flat_map_depth {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::flat_map_depth($a, |v| v.clone(), 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::flat_map_depth($a, $b, 1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::flat_map_depth($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::flat_map_depth($a, $b, $c)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [flat_map_depth!](crate::flat_map_depth!) and read the
/// returned `Value`.
///
/// Macro form: [flat_map_depth_x!](crate::flat_map_depth_x!)
pub fn flat_map_depth_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [flat_map_depth!](crate::flat_map_depth!) and read the
/// returned `Value`.
///
/// Fn form: [flat_map_depth_x()]
#[macro_export]
macro_rules! flat_map_depth_x {
    ($($t:tt)*) => {
        $crate::flat_map_depth_x()
    };
}

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
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(flat_map_depth!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a", 1), json!([0,2,3]));
/// ```
#[macro_export]
macro_rules! flat_map_depth {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::flat_map_depth($a, |v| v.clone(), 1)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::flat_map_depth($a, $crate::iteratee($crate::lib::json!($($__sh)+)), 1)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::flat_map_depth($a, $crate::iteratee($crate::lib::json!($($__sh)+)), 1)
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::flat_map_depth($a, $crate::iteratee($crate::lib::json!($b)), 1)
    };
    ($a:expr, json!($($__sh:tt)+), $c:expr $(,)*) => {
        $crate::flat_map_depth($a, $crate::iteratee($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $c:expr $(,)*) => {
        $crate::flat_map_depth($a, $crate::iteratee($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, $b:literal, $c:expr $(,)*) => {
        $crate::flat_map_depth($a, $crate::iteratee($crate::lib::json!($b)), $c)
    };
    ($a:expr, json!($($__sh:tt)+), $c:expr, $($rest:tt)*) => {
        $crate::flat_map_depth($a, $crate::iteratee($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $c:expr, $($rest:tt)*) => {
        $crate::flat_map_depth($a, $crate::iteratee($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, $b:literal, $c:expr, $($rest:tt)*) => {
        $crate::flat_map_depth($a, $crate::iteratee($crate::lib::json!($b)), $c)
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

build_not_provided_x!(flat_map_depth, flat_map_depth_x);

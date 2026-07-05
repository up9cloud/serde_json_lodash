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

/// See lodash [flatMapDeep](https://lodash.com/docs/#flatMapDeep)
///
/// Maps each element with `iteratee`, then recursively flattens the result
pub fn flat_map_deep(collection: Value, iteratee: fn(&Value) -> Value) -> Value {
    let mut out = vec![];
    for v in collection_values(&collection) {
        flatten_all(iteratee(&v), &mut out);
    }
    Value::Array(out)
}

/// Based on [flat_map_deep()]
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
/// More examples:
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
        json!([])
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

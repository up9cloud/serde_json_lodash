use crate::lib::{json, Value, Map};
use crate::to_string_x;
use crate::collection::collect::collection_values;

/// See lodash [groupBy](https://lodash.com/docs/#groupBy)
///
/// `iteratee` maps each element to a grouping key (coerced to a string)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::group_by;
/// # use serde_json::json;
/// assert_eq!(group_by(json!([6.1, 4.2, 6.3]), |n| json!(n.as_f64().unwrap().floor())), json!({ "4.0": [4.2], "6.0": [6.1, 6.3] }));
/// ```
pub fn group_by(collection: Value, iteratee: fn(&Value) -> Value) -> Value {
    let mut out: Map<String, Value> = Map::new();
    for v in collection_values(&collection) {
        let key = to_string_x(iteratee(&v));
        out.entry(key)
            .or_insert_with(|| json!([]))
            .as_array_mut()
            .unwrap()
            .push(v);
    }
    Value::Object(out)
}

/// Based on [group_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   group_by!(json!([6.1, 4.2, 6.3]), |n| json!(n.as_f64().unwrap().floor())),
///   json!({ "4.0": [4.2], "6.0": [6.1, 6.3] })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(group_by!(), json!({}));
/// assert_eq!(group_by!(json!(["a", "b", "a"])), json!({"a": ["a", "a"], "b": ["b"]}));
/// ```
#[macro_export]
macro_rules! group_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::group_by($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::group_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::group_by($a, $b)
    };
}

use crate::lib::{json, Value, Map};
use crate::to_string_x;
use crate::collection::collect::collection_values;

/// See lodash [countBy](https://lodash.com/docs/#countBy)
///
/// `iteratee` maps each element to a grouping key (coerced to a string)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::count_by;
/// # use serde_json::json;
/// assert_eq!(count_by(json!([6.1, 4.2, 6.3]), |n| json!(n.as_f64().unwrap().floor())), json!({ "4.0": 1, "6.0": 2 }));
/// ```
pub fn count_by(collection: Value, iteratee: fn(&Value) -> Value) -> Value {
    let mut out: Map<String, Value> = Map::new();
    for v in collection_values(&collection) {
        let key = to_string_x(iteratee(&v));
        let entry = out.entry(key).or_insert(json!(0));
        *entry = json!(entry.as_i64().unwrap_or(0) + 1);
    }
    Value::Object(out)
}

/// Based on [count_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   count_by!(json!([6.1, 4.2, 6.3]), |n| json!(n.as_f64().unwrap().floor())),
///   json!({ "4.0": 1, "6.0": 2 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(count_by!(), json!({}));
/// assert_eq!(count_by!(json!(["a", "a", "b"])), json!({"a": 2, "b": 1}));
/// ```
#[macro_export]
macro_rules! count_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::count_by($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::count_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::count_by($a, $b)
    };
}

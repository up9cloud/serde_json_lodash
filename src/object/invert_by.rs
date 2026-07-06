use crate::lib::{json, Value, Map};
use crate::to_string_x;

/// See lodash [invertBy](https://lodash.com/docs/#invertBy)
///
/// `iteratee` transforms each value before it becomes the grouping key
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::invert_by;
/// # use serde_json::json;
/// assert_eq!(invert_by(json!({"a": 1}), |v| v.clone()), json!({"1": ["a"]}));
/// ```
pub fn invert_by(v: Value, iteratee: fn(&Value) -> Value) -> Value {
    let mut out: Map<String, Value> = Map::new();
    let pairs: Vec<(String, Value)> = match v {
        Value::Object(o) => o.into_iter().collect(),
        Value::Array(vec) => vec
            .into_iter()
            .enumerate()
            .map(|(i, val)| (i.to_string(), val))
            .collect(),
        _ => return json!({}),
    };
    for (k, val) in pairs {
        let key = to_string_x(iteratee(&val));
        out.entry(key)
            .or_insert_with(|| json!([]))
            .as_array_mut()
            .unwrap()
            .push(json!(k));
    }
    Value::Object(out)
}

/// Based on [invert_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": 2, "c": 1 });
/// assert_eq!(
///   invert_by!(object, |v| v.clone()),
///   json!({ "1": ["a", "c"], "2": ["b"] })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(invert_by!(), json!({}));
/// assert_eq!(invert_by!(json!({"a": 1})), json!({"1": ["a"]}));
/// ```
#[macro_export]
macro_rules! invert_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::invert_by($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::invert_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::invert_by($a, $b)
    };
}

/// `_x` helper for [invert_by()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [invert_by()] and read the returned `Value`.
pub fn invert_by_x() {
    todo!()
}
/// Based on [invert_by_x()]
#[macro_export]
macro_rules! invert_by_x {
    ($($t:tt)*) => {
        $crate::invert_by_x()
    };
}

use crate::lib::{Map, Value, json};

use crate::to_string_x;

/// Fn form of [invert_by!](crate::invert_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [invert_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::invert_by;
/// # use serde_json::json;
/// assert_eq!(invert_by(json!({"a": 1}), |v| v.clone()), json!({"1": ["a"]}));
/// ```
pub fn invert_by(v: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
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

/// See lodash [invertBy](https://lodash.com/docs/#invertBy)
///
/// `iteratee` transforms each value before it becomes the grouping key
///
/// Fn form: [invert_by()] | `_x` form: **not provided** — see [invert_by_x()]
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
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(invert_by!(json!({"x": {"a": 0, "b": 1}, "y": {"a": 2, "b": 1}}), "a"), json!({"0":["x"],"2":["y"]}));
/// ```
#[macro_export]
macro_rules! invert_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::invert_by($a, |v| v.clone())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::invert_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::invert_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::invert_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::invert_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::invert_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::invert_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::invert_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::invert_by($a, $b)
    };
}

build_not_provided_x!(invert_by, invert_by_x);

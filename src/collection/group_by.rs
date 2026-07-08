use crate::lib::{Map, Value, json};

use crate::to_string_x;

use crate::collection::collect::collection_values;

/// Fn form of [group_by!](crate::group_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [group_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::group_by;
/// # use serde_json::json;
/// assert_eq!(group_by(json!([6.1, 4.2, 6.3]), |n| json!(n.as_f64().unwrap().floor())), json!({ "4.0": [4.2], "6.0": [6.1, 6.3] }));
/// ```
pub fn group_by(collection: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    let mut out: Map<String, Value> = Map::new();
    for v in collection_values(collection) {
        let key = to_string_x(iteratee(&v));
        out.entry(key)
            .or_insert_with(|| json!([]))
            .as_array_mut()
            .unwrap()
            .push(v);
    }
    Value::Object(out)
}

/// See lodash [groupBy](https://lodash.com/docs/#groupBy)
///
/// `iteratee` maps each element to a grouping key (coerced to a string)
///
/// Fn form: [group_by()] | `_x` form: **not provided** — see [group_by_x()]
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
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(group_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!({"0":[{"a":0,"b":1}],"2":[{"a":2,"b":1}],"3":[{"a":3,"b":2}]}));
/// assert_eq!(group_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!({"false":[{"a":3,"b":2}],"true":[{"a":0,"b":1},{"a":2,"b":1}]}));
/// ```
#[macro_export]
macro_rules! group_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::group_by($a, |v| v.clone())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::group_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::group_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::group_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::group_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::group_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::group_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::group_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::group_by($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [group_by!](crate::group_by!) and read the returned
/// `Value`.
///
/// Macro form: [group_by_x!](crate::group_by_x!)
pub fn group_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [group_by!](crate::group_by!) and read the returned
/// `Value`.
///
/// Fn form: [group_by_x()]
#[macro_export]
macro_rules! group_by_x {
    ($($t:tt)*) => {
        $crate::group_by_x()
    };
}

use crate::lib::{Map, Value, json};

use crate::to_string_x;

use crate::collection::collect::collection_values;

/// Fn form of [count_by!](crate::count_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [count_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::count_by;
/// # use serde_json::json;
/// assert_eq!(count_by(json!([6.1, 4.2, 6.3]), |n| json!(n.as_f64().unwrap().floor())), json!({ "4.0": 1, "6.0": 2 }));
/// ```
pub fn count_by(collection: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    let mut out: Map<String, Value> = Map::new();
    for v in collection_values(collection) {
        let key = to_string_x(iteratee(&v));
        let entry = out.entry(key).or_insert(json!(0));
        *entry = json!(entry.as_i64().unwrap_or(0) + 1);
    }
    Value::Object(out)
}

/// See lodash [countBy](https://lodash.com/docs/#countBy)
///
/// `iteratee` maps each element to a grouping key (coerced to a string)
///
/// Fn form: [count_by()] | `_x` form: **not provided** — see [count_by_x()]
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
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(count_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!({"0":1,"2":1,"3":1}));
/// assert_eq!(count_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!({"false":1,"true":2}));
/// ```
#[macro_export]
macro_rules! count_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::count_by($a, |v| v.clone())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::count_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::count_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::count_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::count_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::count_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::count_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::count_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::count_by($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [count_by!](crate::count_by!) and read the returned
/// `Value`.
///
/// Macro form: [count_by_x!](crate::count_by_x!)
pub fn count_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [count_by!](crate::count_by!) and read the returned
/// `Value`.
///
/// Fn form: [count_by_x()]
#[macro_export]
macro_rules! count_by_x {
    ($($t:tt)*) => {
        $crate::count_by_x()
    };
}

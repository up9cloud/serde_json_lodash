use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [map!](crate::map!); see it for the full docs
///
/// `_x` form: **not provided** — see [map_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::map;
/// # use serde_json::json;
/// assert_eq!(map(json!([1, 2]), |n| json!(n.as_i64().unwrap() * 2)), json!([2, 4]));
/// ```
pub fn map(collection: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    Value::Array(collection_values(collection).iter().map(iteratee).collect())
}

/// See lodash [map](https://lodash.com/docs/#map)
///
/// Works on arrays (elements) and objects (values), always returning an array
///
/// Fn form: [map()] | `_x` form: **not provided** — see [map_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   map!(json!([4, 8]), |n| json!(n.as_i64().unwrap().pow(2))),
///   json!([16, 64])
/// );
/// assert_eq!(
///   map!(json!({ "a": 4, "b": 8 }), |n| json!(n.as_i64().unwrap().pow(2))),
///   json!([16, 64])
/// );
/// let users = json!([
///   { "user": "barney" },
///   { "user": "fred" }
/// ]);
/// // The `_.property` iteratee shorthand.
/// assert_eq!(map!(users, "user"), json!(["barney", "fred"]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(map!(), json!([]));
/// assert_eq!(map!(json!([1, 2, 3])), json!([1, 2, 3]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(map!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([0,2,3]));
/// assert_eq!(map!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([true,true,false]));
/// // a fully qualified `serde_json::json!` works as a shorthand too
/// assert_eq!(map!(json!([{"a": 1}]), serde_json::json!("a")), json!([1]));
/// // a shorthand held in a variable must be wrapped explicitly: iteratee
/// // positions take [iteratee()](fn@crate::iteratee) / [property()](fn@crate::property)
/// use serde_json_lodash::property;
/// let path = json!("a");
/// assert_eq!(map!(json!([{"a": 1}, {"a": 2}]), property(path)), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! map {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::map($a, |v| v.clone())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::map($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::map($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::map($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::map($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::map($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::map($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::map($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::map($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [map!](crate::map!) and read the returned `Value`.
///
/// Macro form: [map_x!](crate::map_x!)
pub fn map_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [map!](crate::map!) and read the returned `Value`.
///
/// Fn form: [map_x()]
#[macro_export]
macro_rules! map_x {
    ($($t:tt)*) => {
        $crate::map_x()
    };
}

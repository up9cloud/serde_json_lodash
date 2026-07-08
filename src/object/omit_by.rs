use crate::lib::{Map, Value};

/// Fn form of [omit_by!](crate::omit_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [omit_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::omit_by;
/// # use serde_json::json;
/// assert_eq!(omit_by(json!({"a": 1, "b": "2"}), |v| v.is_number()), json!({"b": "2"}));
/// ```
pub fn omit_by(object: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    let mut out = Map::new();
    if let Value::Object(o) = object {
        for (k, v) in o {
            if !predicate(&v) {
                out.insert(k, v);
            }
        }
    }
    Value::Object(out)
}

/// See lodash [omitBy](https://lodash.com/docs/#omitBy)
///
/// `predicate` is invoked with each property value; matching properties are
/// dropped
///
/// Fn form: [omit_by()] | `_x` form: **not provided** — see [omit_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": "2", "c": 3 });
/// assert_eq!(
///   omit_by!(object, |v| v.is_number()),
///   json!({ "b": "2" })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(omit_by!(), json!({}));
/// assert_eq!(omit_by!(json!({"a": 1})), json!({"a": 1}));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(omit_by!(json!({"x": {"a": 0, "b": 1}, "y": {"a": 2, "b": 1}}), json!({"b": 1})), json!({}));
/// assert_eq!(omit_by!(json!({"x": {"a": 0, "b": 1}, "y": {"a": 2, "b": 1}}), "a"), json!({"x":{"a":0,"b":1}}));
/// ```
#[macro_export]
macro_rules! omit_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::omit_by($a, |v| v.is_null())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::omit_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::omit_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::omit_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::omit_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::omit_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::omit_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::omit_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::omit_by($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [omit_by!](crate::omit_by!) and read the returned
/// `Value`.
///
/// Macro form: [omit_by_x!](crate::omit_by_x!)
pub fn omit_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [omit_by!](crate::omit_by!) and read the returned
/// `Value`.
///
/// Fn form: [omit_by_x()]
#[macro_export]
macro_rules! omit_by_x {
    ($($t:tt)*) => {
        $crate::omit_by_x()
    };
}

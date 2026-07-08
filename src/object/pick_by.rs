use crate::lib::{Map, Value};

/// Fn form of [pick_by!](crate::pick_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [pick_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pick_by;
/// # use serde_json::json;
/// assert_eq!(pick_by(json!({"a": 1, "b": "2"}), |v| v.is_number()), json!({"a": 1}));
/// ```
pub fn pick_by(object: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    let mut out = Map::new();
    if let Value::Object(o) = object {
        for (k, v) in o {
            if predicate(&v) {
                out.insert(k, v);
            }
        }
    }
    Value::Object(out)
}

/// See lodash [pickBy](https://lodash.com/docs/#pickBy)
///
/// `predicate` is invoked with each property value
///
/// Fn form: [pick_by()] | `_x` form: **not provided** — see [pick_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": "2", "c": 3 });
/// assert_eq!(
///   pick_by!(object, |v| v.is_number()),
///   json!({ "a": 1, "c": 3 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pick_by!(), json!({}));
/// assert_eq!(pick_by!(json!({"a": 1})), json!({"a": 1}));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(pick_by!(json!({"x": {"a": 0, "b": 1}, "y": {"a": 2, "b": 1}}), json!({"b": 1})), json!({"x":{"a":0,"b":1},"y":{"a":2,"b":1}}));
/// assert_eq!(pick_by!(json!({"x": {"a": 0, "b": 1}, "y": {"a": 2, "b": 1}}), "a"), json!({"y":{"a":2,"b":1}}));
/// ```
#[macro_export]
macro_rules! pick_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::pick_by($a, |v| !v.is_null())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::pick_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::pick_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::pick_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::pick_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::pick_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::pick_by($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pick_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::pick_by($a, $b)
    };
}

build_not_provided_x!(pick_by, pick_by_x);

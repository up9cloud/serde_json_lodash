use crate::internal::same_value_zero;
use crate::lib::Value;

use crate::internal::property_in;

/// Fn form of [matches_property!](crate::matches_property!); see it for the full docs
///
/// `_x` form: **not provided** — see [matches_property_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::matches_property;
/// # use serde_json::json;
/// assert_eq!(matches_property("a", json!(4))(&json!({"a": 4})), true);
/// ```
pub fn matches_property(
    path: impl Into<Value>,
    src_value: impl Into<Value>,
) -> impl Fn(&Value) -> bool {
    let p_vec = crate::to_path_x(path);
    let src_value = src_value.into();
    move |v| same_value_zero(&property_in(v, &p_vec), &src_value)
}

/// See lodash [matchesProperty](https://lodash.com/docs/#matchesProperty)
///
/// Returns a predicate closure testing whether the value at `path` equals
/// `src_value` (the path is parsed once, up front).
///
/// Fn form: [matches_property()] | `_x` form: **not provided** — see [matches_property_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([
///   { "a": 1, "b": 2, "c": 3 },
///   { "a": 4, "b": 5, "c": 6 }
/// ]);
/// assert_eq!(
///   find!(objects, matches_property!("a", json!(4))),
///   json!({ "a": 4, "b": 5, "c": 6 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(matches_property!("a.b", json!(1))(&json!({"a": {"b": 1}})), true);
/// assert_eq!(matches_property!("x", json!(null))(&json!({"a": 1})), true); // missing path is null
/// // SameValueZero: JS has one number type, so 1 == 1.0
/// assert_eq!(matches_property!("a", json!(1))(&json!({"a": 1.0})), true);
/// ```
#[macro_export]
macro_rules! matches_property {
    ($a:expr, $b:expr $(,)*) => {
        $crate::matches_property($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::matches_property($a, $b)
    };
}

build_not_provided_x!(
    matches_property,
    matches_property_x,
    "The result is a predicate function, which has no primitive form; use [matches_property!](crate::matches_property!) and call the returned closure."
);

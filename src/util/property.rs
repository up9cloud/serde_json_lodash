use crate::lib::Value;

use crate::internal::property_in;

/// Fn form of [property!](crate::property!); see it for the full docs
///
/// `_x` form: **not provided** — see [property_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::property;
/// # use serde_json::json;
/// assert_eq!(property("a.b")(&json!({"a": {"b": 2}})), json!(2));
/// ```
pub fn property(path: impl Into<Value>) -> impl Fn(&Value) -> Value {
    let p_vec = crate::to_path_x(path);
    move |v| property_in(v, &p_vec)
}

/// See lodash [property](https://lodash.com/docs/#property)
///
/// Returns a getter closure for `path` (the path is parsed once, up front).
/// Missing paths yield `Value::Null`.
///
/// Fn form: [property()] | `_x` form: **not provided** — see [property_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([
///   { "a": { "b": 2 } },
///   { "a": { "b": 1 } }
/// ]);
/// assert_eq!(map!(objects.clone(), property!("a.b")), json!([2, 1]));
/// assert_eq!(
///   map!(sort_by!(objects, property!(json!(["a", "b"]))), "a.b"),
///   json!([1, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(property!("x")(&json!({"a": 1})), json!(null));
/// assert_eq!(property!()(&json!({"a": 1})), json!(null));
/// ```
#[macro_export]
macro_rules! property {
    () => {
        $crate::property($crate::lib::json!(null))
    };
    ($a:expr $(,)*) => {
        $crate::property($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::property($a)
    };
}

build_not_provided_x!(
    property,
    property_x,
    "The result is a getter function, which has no primitive form; use [property!](crate::property!) and call the returned closure."
);

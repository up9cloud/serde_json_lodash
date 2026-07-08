use crate::lib::Value;

/// Fn form of [cast_array!](crate::cast_array!); see it for the full docs
///
/// `_x` form: **not provided** — see [cast_array_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::cast_array;
/// # use serde_json::json;
/// assert_eq!(cast_array(json!(1)), json!([1]));
/// ```
pub fn cast_array(v: Value) -> Value {
    match v {
        Value::Array(_) => v,
        _ => Value::Array(vec![v]),
    }
}

/// See lodash [castArray](https://lodash.com/docs/#castArray)
///
/// Fn form: [cast_array()] | `_x` form: **not provided** — see [cast_array_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(cast_array!(json!(1)), json!([1]));
/// assert_eq!(cast_array!(json!({"a": 1})), json!([{"a": 1}]));
/// assert_eq!(cast_array!(json!("abc")), json!(["abc"]));
/// assert_eq!(cast_array!(json!(null)), json!([null]));
/// assert_eq!(cast_array!(), json!([]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(cast_array!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! cast_array {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::cast_array($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::cast_array($a)
    };
}

build_not_provided_x!(cast_array, cast_array_x);

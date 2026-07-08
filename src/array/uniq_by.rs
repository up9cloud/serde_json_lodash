use crate::lib::{Value, json};

use crate::internal::uniq_by_key;

/// Fn form of [uniq_by!](crate::uniq_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [uniq_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::uniq_by;
/// # use serde_json::json;
/// assert_eq!(uniq_by(json!([2.1, 1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())), json!([2.1, 1.2]));
/// ```
pub fn uniq_by(array: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => Value::Array(uniq_by_key(vec, iteratee)),
        _ => json!([]),
    }
}

/// See lodash [uniqBy](https://lodash.com/docs/#uniqBy)
///
/// `iteratee` maps each element to the value used for uniqueness
///
/// Fn form: [uniq_by()] | `_x` form: **not provided** — see [uniq_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   uniq_by!(json!([2.1, 1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([2.1, 1.2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(uniq_by!(), json!([]));
/// assert_eq!(uniq_by!(json!([1, 2, 1])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! uniq_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::uniq_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::uniq_by($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [uniq_by!](crate::uniq_by!) and read the returned
/// `Value`.
///
/// Macro form: [uniq_by_x!](crate::uniq_by_x!)
pub fn uniq_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [uniq_by!](crate::uniq_by!) and read the returned
/// `Value`.
///
/// Fn form: [uniq_by_x()]
#[macro_export]
macro_rules! uniq_by_x {
    ($($t:tt)*) => {
        $crate::uniq_by_x()
    };
}

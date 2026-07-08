use crate::lib::{Value, json};

use crate::object::get::get_in;
use crate::to_path_x;

/// Fn form of [at!](crate::at!); see it for the full docs
///
/// `_x` form: **not provided** — see [at_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::at;
/// # use serde_json::json;
/// assert_eq!(at(json!({"a": 1}), json!(["x"])), json!([null]));
/// ```
pub fn at(object: Value, paths: Value) -> Value {
    match paths {
        Value::Array(keys) => Value::Array(
            keys.into_iter()
                .map(|p| {
                    let p_vec = to_path_x(p);
                    if p_vec.is_empty() {
                        Value::Null
                    } else {
                        get_in(&object, &p_vec).unwrap_or(Value::Null)
                    }
                })
                .collect(),
        ),
        _ => json!([]),
    }
}

/// See lodash [at](https://lodash.com/docs/#at)
///
/// `paths` is an array of path strings; returns the value at each path
///
/// Fn form: [at()] | `_x` form: **not provided** — see [at_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": [{ "b": { "c": 3 } }, 4] });
/// assert_eq!(
///   at!(object, json!(["a[0].b.c", "a[1]"])),
///   json!([3, 4])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(at!(), json!([]));
/// assert_eq!(at!(json!({"a": 1})), json!([]));
/// assert_eq!(at!(json!({"a": 1}), json!(["x"])), json!([null]));
/// ```
#[macro_export]
macro_rules! at {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::at($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::at($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [at!](crate::at!) and read the returned `Value`.
///
/// Macro form: [at_x!](crate::at_x!)
pub fn at_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [at!](crate::at!) and read the returned `Value`.
///
/// Fn form: [at_x()]
#[macro_export]
macro_rules! at_x {
    ($($t:tt)*) => {
        $crate::at_x()
    };
}

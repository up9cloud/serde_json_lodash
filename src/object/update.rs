use crate::lib::Value;

use crate::object::get::get_in;
use crate::{set, to_path_x};

/// Fn form of [update!](crate::update!); see it for the full docs
///
/// `_x` form: **not provided** — see [update_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::update;
/// # use serde_json::json;
/// assert_eq!(update(json!({}), json!("a.b"), |_| json!(1)), json!({"a": {"b": 1}}));
/// ```
pub fn update(object: Value, path: Value, updater: fn(Value) -> Value) -> Value {
    let p_vec = to_path_x(path.clone());
    let current = if p_vec.is_empty() {
        Value::Null
    } else {
        get_in(&object, &p_vec).unwrap_or(Value::Null)
    };
    set(object, path, updater(current))
}

/// See lodash [update](https://lodash.com/docs/#update)
///
/// Updates the value at `path` using the result of `updater(current_value)`
///
/// Fn form: [update()] | `_x` form: **not provided** — see [update_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": [{ "b": { "c": 3 } }] });
/// assert_eq!(
///   update!(object, json!("a[0].b.c"), |n| json!(n.as_i64().unwrap() * 2)),
///   json!({ "a": [{ "b": { "c": 6 } }] })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(update!(), json!(null));
/// assert_eq!(update!(json!({"a": 1})), json!({"a": 1}));
/// assert_eq!(
///   update!(json!({}), json!("a.b"), |_| json!(1)),
///   json!({"a": {"b": 1}})
/// );
/// ```
#[macro_export]
macro_rules! update {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::update($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::update($a, $b, $c)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [update!](crate::update!) and read the returned
/// `Value`.
///
/// Macro form: [update_x!](crate::update_x!)
pub fn update_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [update!](crate::update!) and read the returned
/// `Value`.
///
/// Fn form: [update_x()]
#[macro_export]
macro_rules! update_x {
    ($($t:tt)*) => {
        $crate::update_x()
    };
}

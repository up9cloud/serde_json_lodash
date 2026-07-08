use crate::lib::{Map, Value};

use crate::to_string_x;

use crate::collection::collect::collection_values;

/// Fn form of [key_by!](crate::key_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [key_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::key_by;
/// # use serde_json::json;
/// assert_eq!(key_by(json!(["a", "b"]), |v| v.clone()), json!({"a": "a", "b": "b"}));
/// ```
pub fn key_by(collection: Value, iteratee: fn(&Value) -> Value) -> Value {
    let mut out: Map<String, Value> = Map::new();
    for v in collection_values(&collection) {
        let key = to_string_x(iteratee(&v));
        out.insert(key, v);
    }
    Value::Object(out)
}

/// See lodash [keyBy](https://lodash.com/docs/#keyBy)
///
/// `iteratee` maps each element to its key (coerced to a string)
///
/// Fn form: [key_by()] | `_x` form: **not provided** — see [key_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([
///   { "dir": "left", "code": 97 },
///   { "dir": "right", "code": 100 }
/// ]);
/// assert_eq!(
///   key_by!(array, |o| o["dir"].clone()),
///   json!({
///     "left": { "dir": "left", "code": 97 },
///     "right": { "dir": "right", "code": 100 }
///   })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(key_by!(), json!({}));
/// assert_eq!(key_by!(json!(["a", "b"])), json!({"a": "a", "b": "b"}));
/// ```
#[macro_export]
macro_rules! key_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::key_by($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::key_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::key_by($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [key_by!](crate::key_by!) and read the returned
/// `Value`.
///
/// Macro form: [key_by_x!](crate::key_by_x!)
pub fn key_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [key_by!](crate::key_by!) and read the returned
/// `Value`.
///
/// Fn form: [key_by_x()]
#[macro_export]
macro_rules! key_by_x {
    ($($t:tt)*) => {
        $crate::key_by_x()
    };
}

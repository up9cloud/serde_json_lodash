use crate::lib::{Value, json};

/// Fn form of [to_pairs!](crate::to_pairs!); see it for the full docs
///
/// `_x` form: **not provided** — see [to_pairs_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_pairs;
/// # use serde_json::json;
/// assert_eq!(to_pairs(json!({"a": 1, "b": 2})), json!([["a", 1], ["b", 2]]));
/// ```
pub fn to_pairs(v: Value) -> Value {
    match v {
        Value::Object(o) => Value::Array(o.into_iter().map(|(k, v)| json!([k, v])).collect()),
        Value::Array(vec) => Value::Array(
            vec.into_iter()
                .enumerate()
                .map(|(i, v)| json!([i.to_string(), v]))
                .collect(),
        ),
        Value::String(s) => Value::Array(
            s.chars()
                .enumerate()
                .map(|(i, c)| json!([i.to_string(), c.to_string()]))
                .collect(),
        ),
        _ => json!([]),
    }
}

/// See lodash [toPairs](https://lodash.com/docs/#toPairs)
///
/// Fn form: [to_pairs()] | `_x` form: **not provided** — see [to_pairs_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   to_pairs!(json!({"a": 1, "b": 2})),
///   json!([["a", 1], ["b", 2]])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_pairs!(), json!([]));
/// assert_eq!(to_pairs!(json!(null)), json!([]));
/// assert_eq!(to_pairs!(json!(["a", "b"])), json!([["0", "a"], ["1", "b"]]));
/// ```
#[macro_export]
macro_rules! to_pairs {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_pairs($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_pairs($a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [to_pairs!](crate::to_pairs!) and read the returned
/// `Value`.
///
/// Macro form: [to_pairs_x!](crate::to_pairs_x!)
pub fn to_pairs_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [to_pairs!](crate::to_pairs!) and read the returned
/// `Value`.
///
/// Fn form: [to_pairs_x()]
#[macro_export]
macro_rules! to_pairs_x {
    ($($t:tt)*) => {
        $crate::to_pairs_x()
    };
}

use crate::lib::{json, Value};

/// See lodash [toPairs](https://lodash.com/docs/#toPairs)
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

/// Based on [to_pairs()]
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

/// `_x` helper for [to_pairs()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [to_pairs()] and read the returned `Value`.
pub fn to_pairs_x() {
    todo!()
}
/// Based on [to_pairs_x()]
#[macro_export]
macro_rules! to_pairs_x {
    ($($t:tt)*) => {
        $crate::to_pairs_x()
    };
}

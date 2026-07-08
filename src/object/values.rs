use crate::lib::{Value, json};

/// Fn form of [values!](crate::values!); see it for the full docs
///
/// `_x` form: **not provided** — see [values_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::values;
/// # use serde_json::json;
/// assert_eq!(values(json!({"a": 1, "b": 2})), json!([1, 2]));
/// ```
pub fn values(v: Value) -> Value {
    match v {
        Value::Object(o) => Value::Array(o.into_iter().map(|(_, v)| v).collect()),
        Value::Array(_) => v,
        Value::String(s) => Value::Array(s.chars().map(|c| json!(c.to_string())).collect()),
        _ => json!([]),
    }
}

/// See lodash [values](https://lodash.com/docs/#values)
///
/// Fn form: [values()] | `_x` form: **not provided** — see [values_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(values!(json!({"a": 1, "b": 2})), json!([1, 2]));
/// assert_eq!(values!(json!("hi")), json!(["h", "i"]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(values!(), json!([]));
/// assert_eq!(values!(json!(null)), json!([]));
/// assert_eq!(values!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! values {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::values($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::values($a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [values!](crate::values!) and read the returned
/// `Value`.
///
/// Macro form: [values_x!](crate::values_x!)
pub fn values_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [values!](crate::values!) and read the returned
/// `Value`.
///
/// Fn form: [values_x()]
#[macro_export]
macro_rules! values_x {
    ($($t:tt)*) => {
        $crate::values_x()
    };
}

use crate::lib::{Value, json};

/// Fn form of [head!](crate::head!); see it for the full docs
///
/// `_x` form: **not provided** — see [head_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::head;
/// # use serde_json::json;
/// assert_eq!(head(json!([1, 2, 3])), json!(1));
/// ```
pub fn head(v: Value) -> Value {
    match v {
        Value::Null => json!(null),
        Value::Bool(_) => json!(null),
        Value::String(s) => {
            if s.is_empty() {
                return json!(null);
            }
            json!(s.chars().next())
        }
        Value::Number(_) => json!(null),
        Value::Array(vec) => {
            if vec.is_empty() {
                return json!(null);
            }
            vec.first().unwrap().clone()
        }
        Value::Object(_) => json!(null),
    }
}

/// See lodash [head](https://lodash.com/docs/#head)
///
/// Fn form: [head()] | `_x` form: **not provided** — see [head_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   head!(json!([1, 2, 3])),
///   json!(1)
/// );
/// assert_eq!(
///   head!(json!([])),
///   json!(null)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(head!(), json!(null));
/// assert_eq!(head!(json!(null)), json!(null));
/// assert_eq!(head!(json!(false)), json!(null));
/// assert_eq!(head!(json!(0)), json!(null));
/// assert_eq!(head!(json!("")), json!(null));
/// assert_eq!(head!(json!("ab")), json!("a"));
/// assert_eq!(head!(json!("日本")), json!("日"));
/// assert_eq!(head!(json!({})), json!(null));
/// assert_eq!(head!(json!({"a":1})), json!(null));
/// ```
#[macro_export]
macro_rules! head {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::head($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::head($a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [head!](crate::head!) and read the returned `Value`.
///
/// Macro form: [head_x!](crate::head_x!)
pub fn head_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [head!](crate::head!) and read the returned `Value`.
///
/// Fn form: [head_x()]
#[macro_export]
macro_rules! head_x {
    ($($t:tt)*) => {
        $crate::head_x()
    };
}

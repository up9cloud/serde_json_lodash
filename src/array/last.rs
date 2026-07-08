use crate::lib::{Value, json};

/// Fn form of [last!](crate::last!); see it for the full docs
///
/// `_x` form: **not provided** — see [last_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::last;
/// # use serde_json::json;
/// assert_eq!(last(json!([1, 2, 3])), json!(3));
/// ```
pub fn last(v: Value) -> Value {
    match v {
        Value::Null => json!(null),
        Value::Bool(_) => json!(null),
        Value::String(s) => {
            if s.is_empty() {
                return json!(null);
            }
            json!(s.chars().last())
        }
        Value::Number(_) => json!(null),
        Value::Array(vec) => {
            if vec.is_empty() {
                return json!(null);
            }
            vec.last().unwrap().clone()
        }
        Value::Object(_) => json!(null),
    }
}

/// See lodash [last](https://lodash.com/docs/#last)
///
/// Fn form: [last()] | `_x` form: **not provided** — see [last_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   last!(json!([1, 2, 3])),
///   json!(3)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(last!(), json!(null));
/// assert_eq!(last!(json!(null)), json!(null));
/// assert_eq!(last!(json!(false)), json!(null));
/// assert_eq!(last!(json!(0)), json!(null));
/// assert_eq!(last!(json!("")), json!(null));
/// assert_eq!(last!(json!("ab")), json!("b"));
/// assert_eq!(last!(json!("日本")), json!("本"));
/// assert_eq!(last!(json!([])), json!(null));
/// assert_eq!(last!(json!([null])), json!(null));
/// assert_eq!(last!(json!([[null]])), json!([null]));
/// assert_eq!(last!(json!({})), json!(null));
/// assert_eq!(last!(json!({"a":1})), json!(null));
/// ```
#[macro_export]
macro_rules! last {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::last($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::last($a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [last!](crate::last!) and read the returned `Value`.
///
/// Macro form: [last_x!](crate::last_x!)
pub fn last_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [last!](crate::last!) and read the returned `Value`.
///
/// Fn form: [last_x()]
#[macro_export]
macro_rules! last_x {
    ($($t:tt)*) => {
        $crate::last_x()
    };
}

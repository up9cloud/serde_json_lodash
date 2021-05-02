use crate::lib::{json, Value};

/// See lodash [last](https://lodash.com/docs/#last)
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
/// Based on [last()]
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
/// More examples:
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
        json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::last($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::last($a)
    };
}

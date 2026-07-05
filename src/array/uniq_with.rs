use crate::lib::{json, Value};

/// See lodash [uniqWith](https://lodash.com/docs/#uniqWith)
///
/// `comparator` is invoked to compare elements for uniqueness
pub fn uniq_with(array: Value, comparator: fn(&Value, &Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => {
            let mut out: Vec<Value> = vec![];
            for v in vec {
                if !out.iter().any(|kept| comparator(kept, &v)) {
                    out.push(v);
                }
            }
            Value::Array(out)
        }
        _ => json!([]),
    }
}

/// Based on [uniq_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   uniq_with!(json!([1, 2, 3, 2]), |a, b| a == b),
///   json!([1, 2, 3])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(uniq_with!(), json!([]));
/// assert_eq!(uniq_with!(json!([1, 1])), json!([1, 1]));
/// ```
#[macro_export]
macro_rules! uniq_with {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::uniq_with($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::uniq_with($a, $b)
    };
}

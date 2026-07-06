use crate::lib::{json, Value};

/// See lodash [uniqWith](https://lodash.com/docs/#uniqWith)
///
/// `comparator` is invoked to compare elements for uniqueness
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::uniq_with;
/// # use serde_json::json;
/// assert_eq!(uniq_with(json!([1, 2, 3, 2]), |a, b| a == b), json!([1, 2, 3]));
/// ```
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
/// Additional cases:
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
        $crate::lib::json!([])
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

/// `_x` helper for [uniq_with()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [uniq_with()] and read the returned `Value`.
pub fn uniq_with_x() {
    todo!()
}
/// Based on [uniq_with_x()]
#[macro_export]
macro_rules! uniq_with_x {
    ($($t:tt)*) => {
        $crate::uniq_with_x()
    };
}

use crate::lib::{json, Value};

/// See lodash [intersectionWith](https://lodash.com/docs/#intersectionWith)
///
/// `comparator` is invoked to compare elements
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::intersection_with;
/// # use serde_json::json;
/// assert_eq!(intersection_with(json!([2, 1]), json!([2, 3]), |a, b| a == b), json!([2]));
/// ```
pub fn intersection_with(
    array: Value,
    other: Value,
    comparator: fn(&Value, &Value) -> bool,
) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let b = match other {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let mut out: Vec<Value> = vec![];
    for v in a {
        if b.iter().any(|x| comparator(&v, x)) && !out.iter().any(|x| comparator(&v, x)) {
            out.push(v);
        }
    }
    Value::Array(out)
}

/// Based on [intersection_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   intersection_with!(json!([2, 1]), json!([2, 3]), |a, b| a == b),
///   json!([2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(intersection_with!(), json!([]));
/// assert_eq!(intersection_with!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! intersection_with {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::intersection($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::intersection_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::intersection_with($a, $b, $c)
    };
}

use crate::lib::{json, Value};

/// See lodash [differenceWith](https://lodash.com/docs/#differenceWith)
///
/// `comparator` is invoked to compare elements
pub fn difference_with(
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
        _ => return Value::Array(a),
    };
    Value::Array(
        a.into_iter()
            .filter(|v| !b.iter().any(|x| comparator(v, x)))
            .collect(),
    )
}

/// Based on [difference_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   difference_with!(json!([1, 2, 3]), json!([2, 3]), |a, b| a == b),
///   json!([1])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(difference_with!(), json!([]));
/// assert_eq!(difference_with!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! difference_with {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::difference($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::difference_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::difference_with($a, $b, $c)
    };
}

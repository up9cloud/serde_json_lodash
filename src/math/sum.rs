use crate::lib::{json, Value};
use crate::internal::{f64_to_number, value_to_option_number};

pub(crate) fn sum_values(vec: Vec<Value>) -> Value {
    let total: f64 = vec
        .into_iter()
        .filter_map(|v| value_to_option_number(v).and_then(|n| n.as_f64()))
        .sum();
    match f64_to_number(total) {
        Some(n) => Value::Number(n),
        None => json!(0),
    }
}

/// See lodash [sum](https://lodash.com/docs/#sum)
pub fn sum(array: Value) -> Value {
    match array {
        Value::Array(vec) => sum_values(vec),
        _ => json!(0),
    }
}

/// Based on [sum()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sum!(json!([4, 2, 8, 6])), json!(20));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sum!(), json!(0));
/// assert_eq!(sum!(json!([])), json!(0));
/// assert_eq!(sum!(json!(null)), json!(0));
/// assert_eq!(sum!(json!([1.5, 2.5])), json!(4));
/// ```
#[macro_export]
macro_rules! sum {
    () => {
        json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::sum($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::sum($a)
    };
}

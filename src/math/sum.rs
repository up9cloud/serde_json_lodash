use crate::lib::{Value, json};

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

/// Fn form of [sum!](crate::sum!); see it for the full docs
///
/// `_x` form: **not provided** — see [sum_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sum;
/// # use serde_json::json;
/// assert_eq!(sum(json!([4, 2, 8, 6])), json!(20));
/// ```
pub fn sum(array: Value) -> Value {
    match array {
        Value::Array(vec) => sum_values(vec),
        _ => json!(0),
    }
}

/// See lodash [sum](https://lodash.com/docs/#sum)
///
/// Fn form: [sum()] | `_x` form: **not provided** — see [sum_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sum!(json!([4, 2, 8, 6])), json!(20));
/// ```
///
/// Additional cases:
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
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::sum($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::sum($a)
    };
}

build_not_provided_x!(sum, sum_x);

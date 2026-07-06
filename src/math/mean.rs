use crate::lib::Value;
use crate::internal::{f64_to_number, value_nan, value_to_option_number};

pub(crate) fn mean_values(vec: Vec<Value>) -> Value {
    if vec.is_empty() {
        return value_nan();
    }
    let len = vec.len() as f64;
    let total: f64 = vec
        .into_iter()
        .map(|v| {
            value_to_option_number(v)
                .and_then(|n| n.as_f64())
                .unwrap_or(0.0)
        })
        .sum();
    match f64_to_number(total / len) {
        Some(n) => Value::Number(n),
        None => value_nan(),
    }
}

/// See lodash [mean](https://lodash.com/docs/#mean)
///
/// Returns `Value::Null` (js `NaN`) for an empty array
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::mean;
/// # use serde_json::json;
/// assert_eq!(mean(json!([4, 2, 8, 6])), json!(5));
/// ```
pub fn mean(array: Value) -> Value {
    match array {
        Value::Array(vec) => mean_values(vec),
        _ => value_nan(),
    }
}

/// Based on [mean()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(mean!(json!([4, 2, 8, 6])), json!(5));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(mean!(), json!(f64::NAN));
/// assert_eq!(mean!(json!([])), json!(f64::NAN));
/// assert_eq!(mean!(json!([1, 2])), json!(1.5));
/// ```
#[macro_export]
macro_rules! mean {
    () => {
        $crate::internal::value_nan()
    };
    ($a:expr $(,)*) => {
        $crate::mean($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::mean($a)
    };
}

/// `_x` helper for [mean()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [mean()] and read the returned `Value`.
pub fn mean_x() {
    todo!()
}
/// Based on [mean_x()]
#[macro_export]
macro_rules! mean_x {
    ($($t:tt)*) => {
        $crate::mean_x()
    };
}

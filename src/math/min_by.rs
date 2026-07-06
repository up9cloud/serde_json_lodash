use crate::lib::Value;
use crate::internal::value_to_option_number;

/// See lodash [minBy](https://lodash.com/docs/#minBy)
///
/// `iteratee` maps each element to the value used for comparison
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::min_by;
/// # use serde_json::json;
/// assert_eq!(min_by(json!([3, 1, 2]), |v| v.clone()), json!(1));
/// ```
pub fn min_by(array: Value, iteratee: fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => {
            let mut best: Option<(Value, f64)> = None;
            for v in vec {
                if let Some(n) = value_to_option_number(iteratee(&v)).and_then(|n| n.as_f64()) {
                    match &best {
                        Some((_, bn)) if n >= *bn => {}
                        _ => best = Some((v, n)),
                    }
                }
            }
            best.map(|(v, _)| v).unwrap_or(Value::Null)
        }
        _ => Value::Null,
    }
}

/// Based on [min_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([{ "n": 1 }, { "n": 2 }]);
/// assert_eq!(
///   min_by!(objects, |o| o["n"].clone()),
///   json!({ "n": 1 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(min_by!(), json!(null));
/// assert_eq!(min_by!(json!([])), json!(null));
/// assert_eq!(min_by!(json!([3, 1, 2])), json!(1));
/// ```
#[macro_export]
macro_rules! min_by {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::min($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::min_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::min_by($a, $b)
    };
}

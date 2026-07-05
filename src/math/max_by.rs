use crate::lib::Value;
use crate::internal::value_to_option_number;

/// See lodash [maxBy](https://lodash.com/docs/#maxBy)
///
/// `iteratee` maps each element to the value used for comparison
pub fn max_by(array: Value, iteratee: fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => {
            let mut best: Option<(Value, f64)> = None;
            for v in vec {
                if let Some(n) = value_to_option_number(iteratee(&v)).and_then(|n| n.as_f64()) {
                    match &best {
                        Some((_, bn)) if n <= *bn => {}
                        _ => best = Some((v, n)),
                    }
                }
            }
            best.map(|(v, _)| v).unwrap_or(Value::Null)
        }
        _ => Value::Null,
    }
}

/// Based on [max_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([{ "n": 1 }, { "n": 2 }]);
/// assert_eq!(
///   max_by!(objects, |o| o["n"].clone()),
///   json!({ "n": 2 })
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(max_by!(), json!(null));
/// assert_eq!(max_by!(json!([])), json!(null));
/// assert_eq!(max_by!(json!([1, 2, 3])), json!(3));
/// ```
#[macro_export]
macro_rules! max_by {
    () => {
        json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::max($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::max_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::max_by($a, $b)
    };
}

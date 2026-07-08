use crate::lib::Value;

use crate::internal::value_to_option_number;

/// Fn form of [max_by!](crate::max_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [max_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::max_by;
/// # use serde_json::json;
/// assert_eq!(max_by(json!([1, 2, 3]), |v| v.clone()), json!(3));
/// ```
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

/// See lodash [maxBy](https://lodash.com/docs/#maxBy)
///
/// `iteratee` maps each element to the value used for comparison
///
/// Fn form: [max_by()] | `_x` form: **not provided** — see [max_by_x()]
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
/// Additional cases:
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
        $crate::lib::json!(null)
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [max_by!](crate::max_by!) and read the returned
/// `Value`.
///
/// Macro form: [max_by_x!](crate::max_by_x!)
pub fn max_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [max_by!](crate::max_by!) and read the returned
/// `Value`.
///
/// Fn form: [max_by_x()]
#[macro_export]
macro_rules! max_by_x {
    ($($t:tt)*) => {
        $crate::max_by_x()
    };
}

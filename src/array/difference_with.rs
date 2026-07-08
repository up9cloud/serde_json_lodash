use crate::lib::{Value, json};

/// Fn form of [difference_with!](crate::difference_with!); see it for the full docs
///
/// `_x` form: **not provided** — see [difference_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::difference_with;
/// # use serde_json::json;
/// assert_eq!(difference_with(json!([1, 2, 3]), json!([2, 3]), |a, b| a == b), json!([1]));
/// ```
pub fn difference_with(
    array: Value,
    other: Value,
    comparator: impl Fn(&Value, &Value) -> bool,
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

/// See lodash [differenceWith](https://lodash.com/docs/#differenceWith)
///
/// `comparator` is invoked to compare elements
///
/// Fn form: [difference_with()] | `_x` form: **not provided** — see [difference_with_x()]
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
/// Additional cases:
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
        $crate::lib::json!([])
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

build_not_provided_x!(difference_with, difference_with_x);

use crate::lib::{json, Value};

/// See lodash [differenceBy](https://lodash.com/docs/#differenceBy)
///
/// `iteratee` maps each element to the value used for comparison
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::difference_by;
/// # use serde_json::json;
/// assert_eq!(difference_by(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())), json!([1.2]));
/// ```
pub fn difference_by(array: Value, other: Value, iteratee: fn(&Value) -> Value) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let b = match other {
        Value::Array(v) => v,
        _ => return Value::Array(a),
    };
    let b_keys: Vec<Value> = b.iter().map(iteratee).collect();
    Value::Array(
        a.into_iter()
            .filter(|v| !b_keys.contains(&iteratee(v)))
            .collect(),
    )
}

/// Based on [difference_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   difference_by!(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([1.2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(difference_by!(), json!([]));
/// assert_eq!(difference_by!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! difference_by {
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
        $crate::difference_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::difference_by($a, $b, $c)
    };
}

use crate::lib::{json, Value};

/// See lodash [intersectionBy](https://lodash.com/docs/#intersectionBy)
///
/// `iteratee` maps each element to the value used for comparison
pub fn intersection_by(array: Value, other: Value, iteratee: fn(&Value) -> Value) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let b = match other {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let b_keys: Vec<Value> = b.iter().map(iteratee).collect();
    let mut out = vec![];
    let mut out_keys: Vec<Value> = vec![];
    for v in a {
        let k = iteratee(&v);
        if b_keys.contains(&k) && !out_keys.contains(&k) {
            out_keys.push(k);
            out.push(v);
        }
    }
    Value::Array(out)
}

/// Based on [intersection_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   intersection_by!(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([2.1])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(intersection_by!(), json!([]));
/// assert_eq!(intersection_by!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! intersection_by {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::intersection($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::intersection_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::intersection_by($a, $b, $c)
    };
}

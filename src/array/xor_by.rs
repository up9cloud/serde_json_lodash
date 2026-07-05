use crate::lib::Value;

/// See lodash [xorBy](https://lodash.com/docs/#xorBy)
///
/// `iteratee` maps each element to the value used for comparison
pub fn xor_by(array: Value, other: Value, iteratee: fn(&Value) -> Value) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => vec![],
    };
    let b = match other {
        Value::Array(v) => v,
        _ => vec![],
    };
    let a_keys: Vec<Value> = a.iter().map(iteratee).collect();
    let b_keys: Vec<Value> = b.iter().map(iteratee).collect();
    let mut out = vec![];
    let mut out_keys: Vec<Value> = vec![];
    for (v, k) in a.iter().zip(a_keys.iter()) {
        if !b_keys.contains(k) && !out_keys.contains(k) {
            out.push(v.clone());
            out_keys.push(k.clone());
        }
    }
    for (v, k) in b.iter().zip(b_keys.iter()) {
        if !a_keys.contains(k) && !out_keys.contains(k) {
            out.push(v.clone());
            out_keys.push(k.clone());
        }
    }
    Value::Array(out)
}

/// Based on [xor_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   xor_by!(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([1.2, 3.4])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(xor_by!(), json!([]));
/// assert_eq!(xor_by!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! xor_by {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::xor($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::xor_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::xor_by($a, $b, $c)
    };
}

use crate::lib::Value;

/// Fn form of [xor_with!](crate::xor_with!); see it for the full docs
///
/// `_x` form: **not provided** — see [xor_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::xor_with;
/// # use serde_json::json;
/// assert_eq!(xor_with(json!([2, 1]), json!([2, 3]), |a, b| a == b), json!([1, 3]));
/// ```
pub fn xor_with(array: Value, other: Value, comparator: impl Fn(&Value, &Value) -> bool) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => vec![],
    };
    let b = match other {
        Value::Array(v) => v,
        _ => vec![],
    };
    let mut out: Vec<Value> = vec![];
    for v in a.iter() {
        if !b.iter().any(|x| comparator(v, x)) && !out.iter().any(|x| comparator(v, x)) {
            out.push(v.clone());
        }
    }
    for v in b.iter() {
        if !a.iter().any(|x| comparator(v, x)) && !out.iter().any(|x| comparator(v, x)) {
            out.push(v.clone());
        }
    }
    Value::Array(out)
}

/// See lodash [xorWith](https://lodash.com/docs/#xorWith)
///
/// `comparator` is invoked to compare elements
///
/// Fn form: [xor_with()] | `_x` form: **not provided** — see [xor_with_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   xor_with!(json!([2, 1]), json!([2, 3]), |a, b| a == b),
///   json!([1, 3])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(xor_with!(), json!([]));
/// assert_eq!(xor_with!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! xor_with {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::xor($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::xor_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::xor_with($a, $b, $c)
    };
}

build_not_provided_x!(xor_with, xor_with_x);

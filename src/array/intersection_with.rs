use crate::lib::{Value, json};

/// Fn form of [intersection_with!](crate::intersection_with!); see it for the full docs
///
/// `_x` form: **not provided** — see [intersection_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::intersection_with;
/// # use serde_json::json;
/// assert_eq!(intersection_with(json!([2, 1]), json!([2, 3]), |a, b| a == b), json!([2]));
/// ```
pub fn intersection_with(
    array: Value,
    other: Value,
    comparator: fn(&Value, &Value) -> bool,
) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let b = match other {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let mut out: Vec<Value> = vec![];
    for v in a {
        if b.iter().any(|x| comparator(&v, x)) && !out.iter().any(|x| comparator(&v, x)) {
            out.push(v);
        }
    }
    Value::Array(out)
}

/// See lodash [intersectionWith](https://lodash.com/docs/#intersectionWith)
///
/// `comparator` is invoked to compare elements
///
/// Fn form: [intersection_with()] | `_x` form: **not provided** — see [intersection_with_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   intersection_with!(json!([2, 1]), json!([2, 3]), |a, b| a == b),
///   json!([2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(intersection_with!(), json!([]));
/// assert_eq!(intersection_with!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! intersection_with {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::intersection($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::intersection_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::intersection_with($a, $b, $c)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [intersection_with!](crate::intersection_with!) and
/// read the returned `Value`.
///
/// Macro form: [intersection_with_x!](crate::intersection_with_x!)
pub fn intersection_with_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [intersection_with!](crate::intersection_with!) and
/// read the returned `Value`.
///
/// Fn form: [intersection_with_x()]
#[macro_export]
macro_rules! intersection_with_x {
    ($($t:tt)*) => {
        $crate::intersection_with_x()
    };
}

use crate::lib::{Value, json};

/// Fn form of [intersection_by!](crate::intersection_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [intersection_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::intersection_by;
/// # use serde_json::json;
/// assert_eq!(intersection_by(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())), json!([2.1]));
/// ```
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

/// See lodash [intersectionBy](https://lodash.com/docs/#intersectionBy)
///
/// `iteratee` maps each element to the value used for comparison
///
/// Fn form: [intersection_by()] | `_x` form: **not provided** — see [intersection_by_x()]
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
/// Additional cases:
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
        $crate::lib::json!([])
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [intersection_by!](crate::intersection_by!) and read
/// the returned `Value`.
///
/// Macro form: [intersection_by_x!](crate::intersection_by_x!)
pub fn intersection_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [intersection_by!](crate::intersection_by!) and read
/// the returned `Value`.
///
/// Fn form: [intersection_by_x()]
#[macro_export]
macro_rules! intersection_by_x {
    ($($t:tt)*) => {
        $crate::intersection_by_x()
    };
}

use crate::lib::{Value};
use crate::to_safe_integer_x;

// internal worker for [times()].
fn x_times(n: usize, iteratee: fn(usize) -> Value) -> Value {
    let mut vec = vec![];
    for i in 0..n {
        vec.push(iteratee(i));
    }
    Value::Array(vec)
}

/// See lodash [times](https://lodash.com/docs/#times)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::times;
/// # use serde_json::json;
/// assert_eq!(times(json!(3), |i| json!(i.to_string())), json!(["0","1","2"]));
/// ```
pub fn times<A: Into<Value>>(n: A, iteratee: fn(usize) -> Value) -> Value {
    let n = n.into();
    x_times(to_safe_integer_x(n) as usize, iteratee)
}

/// Based on [times()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   times!(json!(3), |i| json!(i.to_string())),
///   json!(["0","1","2"])
/// );
/// assert_eq!(
///   times!(json!(4), |_| json!(0)),
///   json!([0,0,0,0])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(times!(), json!([]));
/// assert_eq!(times!(json!(null)), json!([]));
/// assert_eq!(times!(json!(false)), json!([]));
/// assert_eq!(times!(json!(0)), json!([]));
/// assert_eq!(times!(json!("")), json!([]));
/// assert_eq!(times!(json!("2")), json!([0,1]));
/// assert_eq!(times!(json!("a")), json!([]));
/// assert_eq!(times!(json!([])), json!([]));
/// assert_eq!(times!(json!([2])), json!([0,1]));
/// assert_eq!(times!(json!([1,2])), json!([]));
/// assert_eq!(times!(json!({})), json!([]));
/// assert_eq!(times!(json!({"a":1})), json!([]));
/// ```
#[macro_export]
macro_rules! times {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::times($a, |x| $crate::lib::json!(x))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::times($a, $b)
    };
}

/// `_x` helper for [times()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [times()] and read the returned `Value`.
pub fn times_x() {
    todo!()
}

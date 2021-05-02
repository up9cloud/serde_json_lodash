use crate::lib::{Value};
use crate::to_safe_integer_x;

///
pub fn x_times(n: usize, iteratee: fn(usize) -> Value) -> Value {
    let mut vec = vec![];
    for i in 0..n {
        vec.push(iteratee(i));
    }
    Value::Array(vec)
}
/// See lodash [times](https://lodash.com/docs/#times)
pub fn times(n: Value, iteratee: fn(usize) -> Value) -> Value {
    x_times(to_safe_integer_x(n) as usize, iteratee)
}

/// Based on [x_times()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   x_times!(3, |i| json!(i.to_string())),
///   json!(["0","1","2"])
/// );
/// assert_eq!(
///   x_times!(4, |_| json!(0)),
///   json!([0,0,0,0])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_times!(), json!([]));
/// assert_eq!(x_times!(0), json!([]));
/// assert_eq!(x_times!(2), json!([0,1]));
/// ```
#[macro_export]
macro_rules! x_times {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::x_times($a, |i| json!(i))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_times($a, $b)
    };
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
/// More examples:
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
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::times($a, |x| json!(x))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::times($a, $b)
    };
}

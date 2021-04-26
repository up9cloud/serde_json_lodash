use crate::lib::{Value};

///
pub fn times(n: usize, iteratee: fn(usize) -> Value) -> Value {
    let mut vec = vec![];
    for i in 0..n {
        vec.push(iteratee(i));
    }
    Value::Array(vec)
}

/// Description can be found in [lodash times](https://lodash.com/docs/#times)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   times!(3, |x| json!(x.to_string())),
///   json!(["0","1","2"])
/// );
/// assert_eq!(
///   times!(4, |_| json!(0)),
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
/// assert_eq!(times!(0), json!([]));
/// assert_eq!(times!(2), json!([0,1]));
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

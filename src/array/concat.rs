use crate::lib::{Value};

/// See lodash [concat](https://lodash.com/docs/#concat)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::concat;
/// # use serde_json::json;
/// assert_eq!(concat(json!(null), json!(null)), json!([null,null]));
/// ```
pub fn concat(v: Value, v2: Value) -> Value {
    let mut result = match v {
        Value::Null => vec![v],
        Value::Bool(_) => vec![v],
        Value::Number(_) => vec![v],
        Value::String(_) => vec![v],
        Value::Array(vec) => vec,
        Value::Object(_) => vec![v],
    };
    match v2 {
        Value::Null => result.push(v2),
        Value::Bool(_) => result.push(v2),
        Value::Number(_) => result.push(v2),
        Value::String(_) => result.push(v2),
        Value::Array(vec) => {
            for v in vec.into_iter() {
                result.push(v)
            }
        }
        Value::Object(_) => result.push(v2),
    };
    Value::Array(result)
}

/// Based on [concat()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([1]);
/// assert_eq!(
///   array,
///   json!([1])
/// );
/// let other = concat!(array, json!(2), json!([3]), json!([[4]]));
/// assert_eq!(
///   other,
///   json!([1, 2, 3, [4]])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(concat!(), json!([]));
/// assert_eq!(concat!(json!(null)), json!([null]));
/// assert_eq!(concat!(json!(false)), json!([false]));
/// assert_eq!(concat!(json!(0)), json!([0]));
/// assert_eq!(concat!(json!("")), json!([""]));
/// assert_eq!(concat!(json!([])), json!([]));
/// assert_eq!(concat!(json!(null),json!(null)), json!([null,null]));
/// ```
#[macro_export]
macro_rules! concat {
    () => (
        $crate::lib::json!([])
    );
    ($a:expr $(,)*) => {
        $crate::concat($a, $crate::lib::json!([]))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::concat($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::concat!($crate::concat($a, $b), $($rest)*)
    };
}

/// `_x` helper for [concat()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [concat()] and read the returned `Value`.
pub fn concat_x() {
    todo!()
}
/// Based on [concat_x()]
#[macro_export]
macro_rules! concat_x {
    ($($t:tt)*) => {
        $crate::concat_x()
    };
}

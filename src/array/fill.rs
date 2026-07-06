use std::mem;

use crate::lib::{json, Value};

/// See lodash [fill](https://lodash.com/docs/#fill)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::fill;
/// # use serde_json::json;
/// assert_eq!(fill(json!([4, 6, 8, 10]), json!("*"), 1, 3), json!([4, "*", "*", 10]));
/// ```
pub fn fill(mut array: Value, value: Value, start: usize, end: usize) -> Value {
    let vec = match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return json!([]);
        }
        Value::Array(ref mut vec) => {
            let real_end = { if end > vec.len() { vec.len() } else { end } };
            for item in vec.iter_mut().take(real_end).skip(start) {
                let _ = mem::replace(item, value.clone());
            }
            vec
        }
    };
    *array.as_array_mut().unwrap() = vec.to_vec();
    array
}

/// Based on [fill()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// //let mut array = json!([1, 2, 3]); // not implemented yet (mutable method...)
/// //let array = fill!(array, json!("a"));
/// //assert_eq!(
/// //  array,
/// //  json!(["a", "a", "a"])
/// //);
/// assert_eq!(
///   fill!(json!([null,null,null]), json!(2)),
///   json!([2, 2, 2])
/// );
/// assert_eq!(
///   fill!(json!([4, 6, 8, 10]), json!("*"), 1, 3),
///   json!([4, "*", "*", 10])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(fill!(), json!([]));
/// assert_eq!(fill!(json!(null)), json!([]));
/// assert_eq!(fill!(json!(true)), json!([]));
/// assert_eq!(fill!(json!(0)), json!([]));
/// assert_eq!(fill!(json!("")), json!([]));
/// assert_eq!(fill!(json!([1])), json!([null]));
/// assert_eq!(fill!(json!([1]), json!(""), 2), json!([1]));
/// assert_eq!(fill!(json!([1]), json!(""), 0, 2), json!([""]));
/// assert_eq!(fill!(json!({})), json!([]));
/// ```
#[macro_export]
macro_rules! fill {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::fill(
            $a,
            $crate::lib::json!(null),
            0,
            $a.as_array().unwrap_or(&vec![]).len(),
        )
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::fill($a, $b, 0, $a.as_array().unwrap_or(&vec![]).len())
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::fill($a, $b, $c, $a.as_array().unwrap_or(&vec![]).len())
    };
    ($a:expr, $b:expr, $c:expr, $d:expr $(,)*) => {
        $crate::fill($a, $b, $c, $d)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $($rest:tt)*) => {
        $crate::fill($a, $b, $c, $d)
    };
}

/// `_x` helper for [fill()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [fill()] and read the returned `Value`.
pub fn fill_x() {
    todo!()
}

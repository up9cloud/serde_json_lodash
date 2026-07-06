use crate::lib::{json, Value};

/// `_x` helper for [index_of()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::index_of_x;
/// # use serde_json::json;
/// assert_eq!(index_of_x(json!([1, 2, 1, 2]), json!(2), 0), 1);
/// ```
pub fn index_of_x(array: Value, value: Value, from_index: usize) -> isize {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => match array {
            Value::Null
            | Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Object(_) => -1,
            Value::Array(vec) => {
                for (i, item) in vec.iter().enumerate().skip(from_index) {
                    if item == &value {
                        return i as isize;
                    }
                }
                -1
            }
        },
        Value::Array(_) | Value::Object(_) => -1,
    }
}
/// See lodash [indexOf](https://lodash.com/docs/#indexOf)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::index_of;
/// # use serde_json::json;
/// assert_eq!(index_of(json!([1, 2, 1, 2]), json!(2), 0), json!(1));
/// ```
pub fn index_of(array: Value, value: Value, from_index: usize) -> Value {
    json!(index_of_x(array, value, from_index))
}

/// Based on [index_of_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(index_of_x!(json!([1, 2, 1, 2]), json!(2), 0), 1);
/// ```
#[macro_export]
macro_rules! index_of_x {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {{ $crate::index_of_x($a, $b, 0) }};
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::index_of_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::index_of_x($a, $b, $c)
    };
}
/// Based on [index_of()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(index_of!(json!([1, 2, 1, 2]), json!(2)), json!(1));
/// assert_eq!(index_of!(json!([1, 2, 1, 2]), json!(2), 2), json!(3));
/// assert_eq!(index_of!(), json!(-1));
/// assert_eq!(index_of!(json!(null)), json!(-1));
/// assert_eq!(index_of!(json!(true)), json!(-1));
/// assert_eq!(index_of!(json!(0)), json!(-1));
/// assert_eq!(index_of!(json!("")), json!(-1));
/// assert_eq!(index_of!(json!([])), json!(-1));
/// assert_eq!(index_of!(json!({})), json!(-1));
/// assert_eq!(index_of!(json!([null]), json!(null)), json!(0));
/// assert_eq!(index_of!(json!([false]), json!(false)), json!(0));
/// assert_eq!(index_of!(json!([0]), json!(0)), json!(0));
/// assert_eq!(index_of!(json!([""]), json!("")), json!(0));
/// assert_eq!(index_of!(json!([[]]), json!([])), json!(-1));
/// assert_eq!(index_of!(json!([[],1]), json!(1)), json!(1));
/// assert_eq!(index_of!(json!([{}]), json!({})), json!(-1));
/// assert_eq!(index_of!(json!([{"a":1}]), json!({"a":1})), json!(-1));
/// assert_eq!(index_of!(json!([{"a":1},1]), json!(1)), json!(1));
/// assert_eq!(index_of!(json!([{"a":1},1,2,1,2]), json!(2)), json!(2));
/// assert_eq!(index_of!(json!([{"a":1},1,2,1,2]), json!(2), 3), json!(4));
/// assert_eq!(index_of!(json!([{"a":1},1,2,1,2]), json!(2), 6), json!(-1));
/// ```
#[macro_export]
macro_rules! index_of {
    () => {
        $crate::lib::json!(-1)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(-1)
    };
    ($a:expr, $b:expr $(,)*) => {{ $crate::index_of($a, $b, 0) }};
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::index_of($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::index_of($a, $b, $c)
    };
}

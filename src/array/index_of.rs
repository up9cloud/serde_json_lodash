use crate::lib::{Value};

/// See lodash [indexOf](https://lodash.com/docs/#indexOf)
pub fn index_of(array: Value, value: Value, from_index: usize) -> isize {
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

/// Based on [index_of()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   index_of!(json!([1, 2, 1, 2]), json!(2)),
///   1
/// );
/// assert_eq!(
///   index_of!(json!([1, 2, 1, 2]), json!(2), 2),
///   3
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(index_of!(), -1);
/// assert_eq!(index_of!(json!(null)), -1);
/// assert_eq!(index_of!(json!(true)), -1);
/// assert_eq!(index_of!(json!(0)), -1);
/// assert_eq!(index_of!(json!("")), -1);
/// assert_eq!(index_of!(json!([])), -1);
/// assert_eq!(index_of!(json!({})), -1);
/// assert_eq!(index_of!(json!([null]), json!(null)), 0);
/// assert_eq!(index_of!(json!([false]), json!(false)), 0);
/// assert_eq!(index_of!(json!([0]), json!(0)), 0);
/// assert_eq!(index_of!(json!([""]), json!("")), 0);
/// assert_eq!(index_of!(json!([[]]), json!([])), -1);
/// assert_eq!(index_of!(json!([[],1]), json!(1)), 1);
/// assert_eq!(index_of!(json!([{}]), json!({})), -1);
/// assert_eq!(index_of!(json!([{"a":1}]), json!({"a":1})), -1);
/// assert_eq!(index_of!(json!([{"a":1},1]), json!(1)), 1);
/// assert_eq!(index_of!(json!([{"a":1},1,2,1,2]), json!(2)), 2);
/// assert_eq!(index_of!(json!([{"a":1},1,2,1,2]), json!(2), 3), 4);
/// assert_eq!(index_of!(json!([{"a":1},1,2,1,2]), json!(2), 6), -1);
/// ```
#[macro_export]
macro_rules! index_of {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {{
        $crate::index_of($a, $b, 0)
    }};
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::index_of($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::index_of($a, $b, $c)
    };
}

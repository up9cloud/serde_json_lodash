use crate::lib::{Value};

///
pub fn last_index_of(array: Value, value: Value, from_index: usize) -> isize {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => match array {
            Value::Null
            | Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Object(_) => -1,
            Value::Array(vec) => {
                for (i, item) in vec.iter().enumerate().take(from_index + 1).rev() {
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

/// Description can be found in [lodash lastIndexOf](https://lodash.com/docs/#lastIndexOf)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   last_index_of!(json!([1, 2, 1, 2]), json!(2)),
///   3
/// );
/// assert_eq!(
///   last_index_of!(json!([1, 2, 1, 2]), json!(2), 2),
///   1
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(last_index_of!(), -1);
/// assert_eq!(last_index_of!(json!(null)), -1);
/// assert_eq!(last_index_of!(json!(true)), -1);
/// assert_eq!(last_index_of!(json!(0)), -1);
/// assert_eq!(last_index_of!(json!("")), -1);
/// assert_eq!(last_index_of!(json!([])), -1);
/// assert_eq!(last_index_of!(json!({})), -1);
/// assert_eq!(last_index_of!(json!([null]), json!(null)), 0);
/// assert_eq!(last_index_of!(json!([false]), json!(false)), 0);
/// assert_eq!(last_index_of!(json!([0]), json!(0)), 0);
/// assert_eq!(last_index_of!(json!([""]), json!("")), 0);
/// assert_eq!(last_index_of!(json!([[]]), json!([])), -1);
/// assert_eq!(last_index_of!(json!([[],1]), json!(1)), 1);
/// assert_eq!(last_index_of!(json!([{}]), json!({})), -1);
/// assert_eq!(last_index_of!(json!([{"a":1}]), json!({"a":1})), -1);
/// assert_eq!(last_index_of!(json!([{"a":1},1]), json!(1)), 1);
/// assert_eq!(last_index_of!(json!([{"a":1},1,2,1,2]), json!(2)), 4);
/// assert_eq!(last_index_of!(json!([{"a":1},1,2,1,2]), json!(2), 3), 2);
/// assert_eq!(last_index_of!(json!([{"a":1},1,2,1,2]), json!(2), 6), 4);
/// assert_eq!(last_index_of!(json!([1,1,1]), json!(1), 2), 2);
/// ```
#[macro_export]
macro_rules! last_index_of {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {{
        let i = $a.as_array().unwrap_or(&vec![]).len();
        $crate::last_index_of($a, $b, i - 1)
    }};
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::last_index_of($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::last_index_of($a, $b, $c)
    };
}

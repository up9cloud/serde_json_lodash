use crate::lib::{json, Value};

/// See lodash [takeRightWhile](https://lodash.com/docs/#takeRightWhile)
///
/// Takes elements from the end while `predicate` returns `true`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::take_right_while;
/// # use serde_json::json;
/// assert_eq!(take_right_while(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() > 2), json!([3, 4]));
/// ```
pub fn take_right_while(array: Value, predicate: fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => {
            let mut n = 0;
            for v in vec.iter().rev() {
                if predicate(v) {
                    n += 1;
                } else {
                    break;
                }
            }
            Value::Array(vec[vec.len() - n..].to_vec())
        }
        _ => json!([]),
    }
}

/// Based on [take_right_while()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   take_right_while!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() > 2),
///   json!([3, 4])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(take_right_while!(), json!([]));
/// assert_eq!(take_right_while!(json!([1, 2, 3])), json!([]));
/// assert_eq!(take_right_while!(json!([1, 2, 3]), |_| true), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! take_right_while {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::take_right_while($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::take_right_while($a, $b)
    };
}

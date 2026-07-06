use crate::lib::{json, Value};

/// See lodash [dropRightWhile](https://lodash.com/docs/#dropRightWhile)
///
/// Drops elements from the end while `predicate` returns `true`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::drop_right_while;
/// # use serde_json::json;
/// assert_eq!(drop_right_while(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() > 2), json!([1, 2]));
/// ```
pub fn drop_right_while(array: Value, predicate: fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(mut vec) => {
            while let Some(last) = vec.last() {
                if predicate(last) {
                    vec.pop();
                } else {
                    break;
                }
            }
            Value::Array(vec)
        }
        _ => json!([]),
    }
}

/// Based on [drop_right_while()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   drop_right_while!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() > 2),
///   json!([1, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(drop_right_while!(), json!([]));
/// assert_eq!(drop_right_while!(json!([1, 2, 3])), json!([1, 2, 3]));
/// assert_eq!(drop_right_while!(json!([1, 2, 3]), |_| true), json!([]));
/// ```
#[macro_export]
macro_rules! drop_right_while {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::drop_right_while($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::drop_right_while($a, $b)
    };
}

/// `_x` helper for [drop_right_while()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [drop_right_while()] and read the returned `Value`.
pub fn drop_right_while_x() {
    todo!()
}

use crate::lib::{json, Value};

///
pub fn drop(v: Value, n: usize) -> Value {
    match v {
        Value::Null => json!([]),
        Value::Bool(_) => json!([]),
        Value::Number(_) => json!([]),
        Value::String(_) => json!([]),
        Value::Array(vec) => {
            if n > vec.len() {
                return json!([]);
            }
            let (_, right) = vec.split_at(n);
            Value::Array(right.to_vec())
        },
        Value::Object(_) => json!([]),
    }
}

/// See [lodash drop](https://lodash.com/docs/#drop)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   drop!(json!([1, 2, 3])),
///   json!([2, 3])
/// );
/// assert_eq!(
///   drop!(json!([1, 2, 3]), 2),
///   json!([3])
/// );
/// assert_eq!(
///   drop!(json!([1, 2, 3]), 5),
///   json!([])
/// );
/// assert_eq!(
///   drop!(json!([1, 2, 3]), 0),
///   json!([1, 2, 3])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(drop!(), json!([]));
/// assert_eq!(drop!(json!(null)), json!([]));
/// assert_eq!(drop!(json!(null), 0, 0), json!([]));
/// assert_eq!(drop!(json!(true)), json!([]));
/// assert_eq!(drop!(json!(0)), json!([]));
/// assert_eq!(drop!(json!("")), json!([]));
/// assert_eq!(drop!(json!([])), json!([]));
/// assert_eq!(drop!(json!({})), json!([]));
/// ```
#[macro_export]
macro_rules! drop {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::drop($a, 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::drop($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::drop($a, $b)
    };
}

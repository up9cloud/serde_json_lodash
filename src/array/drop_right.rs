use crate::lib::{json, Value};

/// See lodash [dropRight](https://lodash.com/docs/#dropRight)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::drop_right;
/// # use serde_json::json;
/// assert_eq!(drop_right(json!([1, 2, 3]), 2), json!([1]));
/// ```
pub fn drop_right(v: Value, n: usize) -> Value {
    match v {
        Value::Null => json!([]),
        Value::Bool(_) => json!([]),
        Value::Number(_) => json!([]),
        Value::String(_) => json!([]),
        Value::Array(vec) => {
            let len = vec.len();
            if n > len {
                return json!([]);
            }
            let (left, _) = vec.split_at(len - n);
            Value::Array(left.to_vec())
        }
        Value::Object(_) => json!([]),
    }
}

/// Based on [drop_right()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   drop_right!(json!([1, 2, 3])),
///   json!([1, 2])
/// );
/// assert_eq!(
///   drop_right!(json!([1, 2, 3]), 2),
///   json!([1])
/// );
/// assert_eq!(
///   drop_right!(json!([1, 2, 3]), 5),
///   json!([])
/// );
/// assert_eq!(
///   drop_right!(json!([1, 2, 3]), 0),
///   json!([1, 2, 3])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(drop_right!(), json!([]));
/// assert_eq!(drop_right!(json!(null)), json!([]));
/// assert_eq!(drop_right!(json!(null), 0, 0), json!([]));
/// assert_eq!(drop_right!(json!(true)), json!([]));
/// assert_eq!(drop_right!(json!(0)), json!([]));
/// assert_eq!(drop_right!(json!("")), json!([]));
/// assert_eq!(drop_right!(json!([])), json!([]));
/// assert_eq!(drop_right!(json!({})), json!([]));
/// ```
#[macro_export]
macro_rules! drop_right {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::drop_right($a, 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::drop_right($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::drop_right($a, $b)
    };
}

/// `_x` helper for [drop_right()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [drop_right()] and read the returned `Value`.
pub fn drop_right_x() {
    todo!()
}

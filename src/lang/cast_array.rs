use crate::lib::Value;

/// See lodash [castArray](https://lodash.com/docs/#castArray)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::cast_array;
/// # use serde_json::json;
/// assert_eq!(cast_array(json!(1)), json!([1]));
/// ```
pub fn cast_array(v: Value) -> Value {
    match v {
        Value::Array(_) => v,
        _ => Value::Array(vec![v]),
    }
}

/// Based on [cast_array()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(cast_array!(json!(1)), json!([1]));
/// assert_eq!(cast_array!(json!({"a": 1})), json!([{"a": 1}]));
/// assert_eq!(cast_array!(json!("abc")), json!(["abc"]));
/// assert_eq!(cast_array!(json!(null)), json!([null]));
/// assert_eq!(cast_array!(), json!([]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(cast_array!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! cast_array {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::cast_array($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::cast_array($a)
    };
}

/// `_x` helper for [cast_array()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [cast_array()] and read the returned `Value`.
pub fn cast_array_x() {
    todo!()
}
/// Based on [cast_array_x()]
#[macro_export]
macro_rules! cast_array_x {
    ($($t:tt)*) => {
        $crate::cast_array_x()
    };
}

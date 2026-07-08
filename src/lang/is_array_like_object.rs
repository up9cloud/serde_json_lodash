use crate::lib::{Value, json};

/// Fn form of [is_array_like_object!](crate::is_array_like_object!); see it for the full docs
///
/// `_x` forms: [is_array_like_object_x!](crate::is_array_like_object_x!), [is_array_like_object_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_like_object;
/// # use serde_json::json;
/// assert_eq!(is_array_like_object(&json!([1, 2, 3])), json!(true));
/// ```
pub fn is_array_like_object(v: &Value) -> Value {
    json!(is_array_like_object_x(v))
}

/// See lodash [isArrayLikeObject](https://lodash.com/docs/#isArrayLikeObject)
///
/// Fn form: [is_array_like_object()] | `_x` forms: [is_array_like_object_x!](crate::is_array_like_object_x!), [is_array_like_object_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_array_like_object!(&json!([1, 2, 3])), json!(true));
/// assert_eq!(is_array_like_object!(&json!("abc")), json!(false));
/// assert_eq!(is_array_like_object!(), json!(false));
/// assert_eq!(is_array_like_object!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_array_like_object {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_array_like_object($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_like_object($a)
    };
}

/// `_x` helper for [is_array_like_object!](crate::is_array_like_object!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_array_like_object_x!](crate::is_array_like_object_x!) | `Value` forms: [is_array_like_object!](crate::is_array_like_object!), [is_array_like_object()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_like_object_x;
/// # use serde_json::json;
/// assert_eq!(is_array_like_object_x(&json!([1, 2, 3])), true);
/// ```
pub fn is_array_like_object_x(v: &Value) -> bool {
    v.is_array()
}

/// `_x` helper for [is_array_like_object!](crate::is_array_like_object!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_array_like_object_x()] | `Value` forms: [is_array_like_object!](crate::is_array_like_object!), [is_array_like_object()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_like_object_x!(&json!([1, 2, 3])), true);
/// ```
#[macro_export]
macro_rules! is_array_like_object_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_array_like_object_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_like_object_x($a)
    };
}

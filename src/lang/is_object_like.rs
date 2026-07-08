use crate::lib::{Value, json};

/// Fn form of [is_object_like!](crate::is_object_like!); see it for the full docs
///
/// `_x` forms: [is_object_like_x!](crate::is_object_like_x!), [is_object_like_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_object_like;
/// # use serde_json::json;
/// assert_eq!(is_object_like(&json!({})), json!(true));
/// ```
pub fn is_object_like(v: &Value) -> Value {
    json!(is_object_like_x(v))
}

/// See lodash [isObjectLike](https://lodash.com/docs/#isObjectLike)
///
/// Fn form: [is_object_like()] | `_x` forms: [is_object_like_x!](crate::is_object_like_x!), [is_object_like_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_object_like!(&json!({})), json!(true));
/// assert_eq!(is_object_like!(&json!([1, 2, 3])), json!(true));
/// assert_eq!(is_object_like!(&json!(null)), json!(false));
/// assert_eq!(is_object_like!(), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_object_like!(), json!(false));
/// assert_eq!(is_object_like!(&json!(null)), json!(false));
/// assert_eq!(is_object_like!(&json!(true)), json!(false));
/// assert_eq!(is_object_like!(&json!(0)), json!(false));
/// assert_eq!(is_object_like!(&json!("ab")), json!(false));
/// assert_eq!(is_object_like!(&json!([1, 2])), json!(true));
/// assert_eq!(is_object_like!(&json!({"a": 1})), json!(true));
/// assert_eq!(is_object_like!(&json!("abc")), json!(false));
/// ```
#[macro_export]
macro_rules! is_object_like {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_object_like($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_object_like($a)
    };
}

/// `_x` helper for [is_object_like!](crate::is_object_like!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_object_like_x!](crate::is_object_like_x!) | `Value` forms: [is_object_like!](crate::is_object_like!), [is_object_like()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_object_like_x;
/// # use serde_json::json;
/// assert_eq!(is_object_like_x(&json!({})), true);
/// ```
pub fn is_object_like_x(v: &Value) -> bool {
    v.is_object() || v.is_array()
}

/// `_x` helper for [is_object_like!](crate::is_object_like!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_object_like_x()] | `Value` forms: [is_object_like!](crate::is_object_like!), [is_object_like()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_object_like_x!(&json!({})), true);
/// ```
#[macro_export]
macro_rules! is_object_like_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_object_like_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_object_like_x($a)
    };
}

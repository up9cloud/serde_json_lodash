use crate::lib::{Value, json};

/// Fn form of [is_array_like!](crate::is_array_like!); see it for the full docs
///
/// `_x` forms: [is_array_like_x!](crate::is_array_like_x!), [is_array_like_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_like;
/// # use serde_json::json;
/// assert_eq!(is_array_like(&json!([1, 2, 3])), json!(true));
/// ```
pub fn is_array_like(v: &Value) -> Value {
    json!(is_array_like_x(v))
}

/// See lodash [isArrayLike](https://lodash.com/docs/#isArrayLike)
///
/// Fn form: [is_array_like()] | `_x` forms: [is_array_like_x!](crate::is_array_like_x!), [is_array_like_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_array_like!(&json!([1, 2, 3])), json!(true));
/// assert_eq!(is_array_like!(&json!("abc")), json!(true));
/// assert_eq!(is_array_like!(&json!({})), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_like!(), json!(false));
/// assert_eq!(is_array_like!(&json!(null)), json!(false));
/// assert_eq!(is_array_like!(&json!(true)), json!(false));
/// assert_eq!(is_array_like!(&json!(0)), json!(false));
/// assert_eq!(is_array_like!(&json!("ab")), json!(true));
/// assert_eq!(is_array_like!(&json!([1, 2])), json!(true));
/// assert_eq!(is_array_like!(&json!({"a": 1})), json!(false));
/// ```
#[macro_export]
macro_rules! is_array_like {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_array_like($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_like($a)
    };
}

/// `_x` helper for [is_array_like!](crate::is_array_like!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_array_like_x!](crate::is_array_like_x!) | `Value` forms: [is_array_like!](crate::is_array_like!), [is_array_like()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_like_x;
/// # use serde_json::json;
/// assert_eq!(is_array_like_x(&json!([1, 2, 3])), true);
/// ```
pub fn is_array_like_x(v: &Value) -> bool {
    v.is_array() || v.is_string()
}

/// `_x` helper for [is_array_like!](crate::is_array_like!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_array_like_x()] | `Value` forms: [is_array_like!](crate::is_array_like!), [is_array_like()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_like_x!(&json!([1, 2, 3])), true);
/// ```
#[macro_export]
macro_rules! is_array_like_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_array_like_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_like_x($a)
    };
}

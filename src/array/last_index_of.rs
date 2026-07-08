use crate::lib::{Value, json};

use crate::internal::resolve_from_index_back;

/// Fn form of [last_index_of!](crate::last_index_of!); see it for the full docs
///
/// `_x` forms: [last_index_of_x!](crate::last_index_of_x!), [last_index_of_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::last_index_of;
/// # use serde_json::json;
/// assert_eq!(last_index_of(json!([1, 2, 1, 2]), json!(2), 3), json!(3));
/// ```
pub fn last_index_of(array: Value, value: Value, from_index: isize) -> Value {
    json!(last_index_of_x(array, value, from_index))
}

/// See lodash [lastIndexOf](https://lodash.com/docs/#lastIndexOf)
///
/// Fn form: [last_index_of()] | `_x` forms: [last_index_of_x!](crate::last_index_of_x!), [last_index_of_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(last_index_of!(json!([1, 2, 1, 2]), json!(2)), json!(3));
/// assert_eq!(last_index_of!(json!([1, 2, 1, 2]), json!(2), 2), json!(1));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(last_index_of!(), json!(-1));
/// assert_eq!(last_index_of!(json!(null)), json!(-1));
/// assert_eq!(last_index_of!(json!({"a": 1})), json!(-1));
/// assert_eq!(last_index_of!(json!(null), json!(null)), json!(-1));
/// assert_eq!(last_index_of!(json!(1), json!(1)), json!(-1));
/// assert_eq!(last_index_of!(json!(1), json!(2)), json!(-1));
/// assert_eq!(last_index_of!(json!([1, 2, 3]), json!(2)), json!(1));
/// assert_eq!(last_index_of!(json!("abc"), json!("bc")), json!(-1));
/// assert_eq!(last_index_of!(json!(true)), json!(-1));
/// assert_eq!(last_index_of!(json!(0)), json!(-1));
/// assert_eq!(last_index_of!(json!("")), json!(-1));
/// assert_eq!(last_index_of!(json!([])), json!(-1));
/// assert_eq!(last_index_of!(json!({})), json!(-1));
/// assert_eq!(last_index_of!(json!([null]), json!(null)), json!(0));
/// assert_eq!(last_index_of!(json!([false]), json!(false)), json!(0));
/// assert_eq!(last_index_of!(json!([0]), json!(0)), json!(0));
/// assert_eq!(last_index_of!(json!([""]), json!("")), json!(0));
/// assert_eq!(last_index_of!(json!([[]]), json!([])), json!(-1));
/// assert_eq!(last_index_of!(json!([[],1]), json!(1)), json!(1));
/// assert_eq!(last_index_of!(json!([{}]), json!({})), json!(-1));
/// assert_eq!(last_index_of!(json!([{"a":1}]), json!({"a":1})), json!(-1));
/// assert_eq!(last_index_of!(json!([{"a":1},1]), json!(1)), json!(1));
/// assert_eq!(last_index_of!(json!([{"a":1},1,2,1,2]), json!(2)), json!(4));
/// assert_eq!(last_index_of!(json!([{"a":1},1,2,1,2]), json!(2), 3), json!(2));
/// assert_eq!(last_index_of!(json!([{"a":1},1,2,1,2]), json!(2), 6), json!(4));
/// assert_eq!(last_index_of!(json!([1,1,1]), json!(1), 2), json!(2));
/// // negative fromIndex counts back from the end
/// assert_eq!(last_index_of!(json!([1, 2, 1, 2]), json!(2), -2), json!(1));
/// assert_eq!(last_index_of!(json!([1, 2, 1, 2]), json!(2), 9), json!(3));
/// ```
#[macro_export]
macro_rules! last_index_of {
    () => {
        $crate::lib::json!(-1)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(-1)
    };
    ($a:expr, $b:expr $(,)*) => {
        // -1 resolves to the last element, lodash's default fromIndex
        $crate::last_index_of($a, $b, -1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::last_index_of($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::last_index_of($a, $b, $c)
    };
}

/// `_x` helper for [last_index_of!](crate::last_index_of!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [last_index_of_x!](crate::last_index_of_x!) | `Value` forms: [last_index_of!](crate::last_index_of!), [last_index_of()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::last_index_of_x;
/// # use serde_json::json;
/// assert_eq!(last_index_of_x(json!([1, 2, 1, 2]), json!(2), 3), 3);
/// ```
pub fn last_index_of_x(array: Value, value: Value, from_index: isize) -> isize {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => match array {
            Value::Null
            | Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Object(_) => -1,
            Value::Array(vec) => {
                let start = resolve_from_index_back(vec.len(), from_index);
                for (i, item) in vec.iter().enumerate().take(start + 1).rev() {
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

/// `_x` helper for [last_index_of!](crate::last_index_of!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [last_index_of_x()] | `Value` forms: [last_index_of!](crate::last_index_of!), [last_index_of()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(last_index_of_x!(json!([1, 2, 1, 2]), json!(2), 3), 3);
/// ```
#[macro_export]
macro_rules! last_index_of_x {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {{
        let i = $a.as_array().unwrap_or(&vec![]).len();
        $crate::last_index_of_x($a, $b, i - 1)
    }};
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::last_index_of_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::last_index_of_x($a, $b, $c)
    };
}

use crate::lib::{Value, json};

use crate::internal::compare_values;

/// Fn form of [lte!](crate::lte!); see it for the full docs
///
/// `_x` forms: [lte_x!](crate::lte_x!), [lte_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::lte;
/// # use serde_json::json;
/// assert_eq!(lte(&json!(1), &json!(3)), json!(true));
/// ```
pub fn lte(a: &Value, b: &Value) -> Value {
    json!(lte_x(a, b))
}

/// See lodash [lte](https://lodash.com/docs/#lte)
///
/// Fn form: [lte()] | `_x` forms: [lte_x!](crate::lte_x!), [lte_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(lte!(&json!(1), &json!(3)), json!(true));
/// assert_eq!(lte!(&json!(3), &json!(3)), json!(true));
/// assert_eq!(lte!(&json!(3), &json!(1)), json!(false));
/// assert_eq!(lte!(), json!(false));
/// assert_eq!(lte!(&json!(1)), json!(false));
/// assert_eq!(lte!(&json!("a"), &json!("a")), json!(true));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lte!(), json!(false));
/// assert_eq!(lte!(json!(null)), json!(false));
/// assert_eq!(lte!(json!({"a": 1})), json!(false));
/// assert_eq!(lte!(&json!(null), &json!(null)), json!(true));
/// assert_eq!(lte!(&json!(1), &json!(1)), json!(true));
/// assert_eq!(lte!(&json!(1), &json!(2)), json!(true));
/// assert_eq!(lte!(&json!([1, 2, 3]), &json!(2)), json!(false));
/// assert_eq!(lte!(&json!("abc"), &json!("bc")), json!(true));
/// ```
#[macro_export]
macro_rules! lte {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::lte($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::lte($a, $b)
    };
}

/// `_x` helper for [lte!](crate::lte!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [lte_x!](crate::lte_x!) | `Value` forms: [lte!](crate::lte!), [lte()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::lte_x;
/// # use serde_json::json;
/// assert_eq!(lte_x(&json!(1), &json!(3)), true);
/// ```
pub fn lte_x(a: &Value, b: &Value) -> bool {
    matches!(
        compare_values(a, b),
        Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal)
    )
}

/// `_x` helper for [lte!](crate::lte!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [lte_x()] | `Value` forms: [lte!](crate::lte!), [lte()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lte_x!(&json!(1), &json!(3)), true);
/// ```
#[macro_export]
macro_rules! lte_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::lte_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::lte_x($a, $b)
    };
}

use crate::lib::{json, Value};
use crate::internal::compare_values;

/// `_x` helper for [lte()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
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
/// See lodash [lte](https://lodash.com/docs/#lte)
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

/// Based on [lte_x()]
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
/// Based on [lte()]
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

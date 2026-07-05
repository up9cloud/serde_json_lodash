use crate::lib::Value;
use crate::internal::compare_values;

/// See lodash [gte](https://lodash.com/docs/#gte)
pub fn gte(a: &Value, b: &Value) -> bool {
    matches!(
        compare_values(a, b),
        Some(std::cmp::Ordering::Greater) | Some(std::cmp::Ordering::Equal)
    )
}

/// Based on [gte()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(gte!(&json!(3), &json!(1)), true);
/// assert_eq!(gte!(&json!(3), &json!(3)), true);
/// assert_eq!(gte!(&json!(1), &json!(3)), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(gte!(), false);
/// assert_eq!(gte!(&json!(1)), false);
/// assert_eq!(gte!(&json!("a"), &json!("a")), true);
/// ```
#[macro_export]
macro_rules! gte {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::gte($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::gte($a, $b)
    };
}

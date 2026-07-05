use crate::lib::Value;
use crate::internal::compare_values;

/// See lodash [lte](https://lodash.com/docs/#lte)
pub fn lte(a: &Value, b: &Value) -> bool {
    matches!(
        compare_values(a, b),
        Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal)
    )
}

/// Based on [lte()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(lte!(&json!(1), &json!(3)), true);
/// assert_eq!(lte!(&json!(3), &json!(3)), true);
/// assert_eq!(lte!(&json!(3), &json!(1)), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lte!(), false);
/// assert_eq!(lte!(&json!(1)), false);
/// assert_eq!(lte!(&json!("a"), &json!("a")), true);
/// ```
#[macro_export]
macro_rules! lte {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::lte($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::lte($a, $b)
    };
}

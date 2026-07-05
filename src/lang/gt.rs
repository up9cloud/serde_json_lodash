use crate::lib::Value;
use crate::internal::compare_values;

/// See lodash [gt](https://lodash.com/docs/#gt)
pub fn gt(a: &Value, b: &Value) -> bool {
    matches!(compare_values(a, b), Some(std::cmp::Ordering::Greater))
}

/// Based on [gt()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(gt!(&json!(3), &json!(1)), true);
/// assert_eq!(gt!(&json!(3), &json!(3)), false);
/// assert_eq!(gt!(&json!(1), &json!(3)), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(gt!(), false);
/// assert_eq!(gt!(&json!(1)), false);
/// assert_eq!(gt!(&json!("b"), &json!("a")), true);
/// assert_eq!(gt!(&json!("3"), &json!(1)), true);
/// assert_eq!(gt!(&json!({}), &json!(1)), false);
/// ```
#[macro_export]
macro_rules! gt {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::gt($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::gt($a, $b)
    };
}

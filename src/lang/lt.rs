use crate::lib::Value;
use crate::internal::compare_values;

/// See lodash [lt](https://lodash.com/docs/#lt)
pub fn lt(a: &Value, b: &Value) -> bool {
    matches!(compare_values(a, b), Some(std::cmp::Ordering::Less))
}

/// Based on [lt()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(lt!(&json!(1), &json!(3)), true);
/// assert_eq!(lt!(&json!(3), &json!(3)), false);
/// assert_eq!(lt!(&json!(3), &json!(1)), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lt!(), false);
/// assert_eq!(lt!(&json!(1)), false);
/// assert_eq!(lt!(&json!("a"), &json!("b")), true);
/// ```
#[macro_export]
macro_rules! lt {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::lt($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::lt($a, $b)
    };
}

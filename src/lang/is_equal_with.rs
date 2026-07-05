use crate::lib::Value;

/// See lodash [isEqualWith](https://lodash.com/docs/#isEqualWith)
///
/// If `customizer` returns `None`, comparison falls back to [is_equal()](fn@crate::is_equal)
pub fn is_equal_with(a: &Value, b: &Value, customizer: fn(&Value, &Value) -> Option<bool>) -> bool {
    match customizer(a, b) {
        Some(result) => result,
        None => a == b,
    }
}

/// Based on [is_equal_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// use serde_json::Value;
/// fn customizer(a: &Value, b: &Value) -> Option<bool> {
///   match (a.as_str(), b.as_str()) {
///     (Some(a), Some(b)) => Some(a.to_lowercase() == b.to_lowercase()),
///     _ => None,
///   }
/// }
/// assert_eq!(is_equal_with!(&json!("Hello"), &json!("hello"), customizer), true);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_equal_with!(), true);
/// assert_eq!(is_equal_with!(&json!(1)), false);
/// assert_eq!(is_equal_with!(&json!(1), &json!(1)), true);
/// assert_eq!(is_equal_with!(&json!(1), &json!(2), |_, _| Some(true)), true);
/// ```
#[macro_export]
macro_rules! is_equal_with {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_equal($a, &serde_json::json!(null))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_equal($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::is_equal_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::is_equal_with($a, $b, $c)
    };
}

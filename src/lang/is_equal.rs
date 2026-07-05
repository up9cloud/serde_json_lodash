use crate::lib::Value;

/// See lodash [isEqual](https://lodash.com/docs/#isEqual)
pub fn is_equal(a: &Value, b: &Value) -> bool {
    a == b
}

/// Based on [is_equal()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1 });
/// let other = json!({ "a": 1 });
/// assert_eq!(is_equal!(&object, &other), true);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_equal!(), true);
/// assert_eq!(is_equal!(&json!(1)), false);
/// assert_eq!(is_equal!(&json!([1, [2]]), &json!([1, [2]])), true);
/// assert_eq!(is_equal!(&json!(1), &json!("1")), false);
/// ```
#[macro_export]
macro_rules! is_equal {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_equal($a, &serde_json::json!(null))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_equal($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::is_equal($a, $b)
    };
}

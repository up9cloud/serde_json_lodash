use crate::lib::Value;

/// See lodash [eq](https://lodash.com/docs/#eq)
///
/// *Note:* JS reference identity cannot be expressed with owned
/// `serde_json::Value`, so this is a value (deep) comparison
pub fn eq(a: &Value, b: &Value) -> bool {
    a == b
}

/// Based on [eq()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1 });
/// let other = json!({ "a": 1 });
/// assert_eq!(eq!(&object, &object), true);
/// assert_eq!(eq!(&object, &other), true); // js version is false, reference comparison is not portable
/// assert_eq!(eq!(&json!("a"), &json!("a")), true);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(eq!(), true);
/// assert_eq!(eq!(&json!(null)), true); // eq(null, undefined) => js loose equality is true
/// assert_eq!(eq!(&json!(1), &json!("1")), false);
/// ```
#[macro_export]
macro_rules! eq {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::eq($a, &serde_json::json!(null))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::eq($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::eq($a, $b)
    };
}

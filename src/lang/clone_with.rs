use crate::lib::Value;

/// See lodash [cloneWith](https://lodash.com/docs/#cloneWith)
///
/// If `customizer` returns `None` the value is cloned as [clone()](fn@crate::clone) would
pub fn clone_with(v: &Value, customizer: fn(&Value) -> Option<Value>) -> Value {
    match customizer(v) {
        Some(result) => result,
        None => v.clone(),
    }
}

/// Based on [clone_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// fn customizer(v: &Value) -> Option<Value> {
///   if v.is_array() { Some(json!("array!")) } else { None }
/// }
/// use serde_json::Value;
/// assert_eq!(clone_with!(&json!([1]), customizer), json!("array!"));
/// assert_eq!(clone_with!(&json!(1), customizer), json!(1));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(clone_with!(), json!(null));
/// assert_eq!(clone_with!(&json!([1])), json!([1]));
/// ```
#[macro_export]
macro_rules! clone_with {
    () => {
        json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::clone($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::clone_with($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::clone_with($a, $b)
    };
}

use crate::lib::Value;

/// Fn form of [clone_with!](crate::clone_with!); see it for the full docs
///
/// `_x` form: **not provided** — see [clone_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::clone_with;
/// # use serde_json::json;
/// assert_eq!(clone_with(&json!([1]), |_| None), json!([1]));
/// ```
pub fn clone_with(v: &Value, customizer: impl Fn(&Value) -> Option<Value>) -> Value {
    match customizer(v) {
        Some(result) => result,
        None => v.clone(),
    }
}

/// See lodash [cloneWith](https://lodash.com/docs/#cloneWith)
///
/// If `customizer` returns `None` the value is cloned as [clone()](fn@crate::clone) would
///
/// Fn form: [clone_with()] | `_x` form: **not provided** — see [clone_with_x()]
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
/// Additional cases:
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
        $crate::lib::json!(null)
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [clone_with!](crate::clone_with!) and read the returned
/// `Value`.
///
/// Macro form: [clone_with_x!](crate::clone_with_x!)
pub fn clone_with_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [clone_with!](crate::clone_with!) and read the returned
/// `Value`.
///
/// Fn form: [clone_with_x()]
#[macro_export]
macro_rules! clone_with_x {
    ($($t:tt)*) => {
        $crate::clone_with_x()
    };
}

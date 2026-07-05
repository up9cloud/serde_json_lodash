use crate::lib::Value;

/// See lodash [clone](https://lodash.com/docs/#clone)
///
/// *Note:* JS shallow-copy semantic (sharing references) cannot be expressed
/// with owned `serde_json::Value`, so this is effectively a full copy
pub fn clone(v: &Value) -> Value {
    v.clone()
}

/// Based on [clone()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([{ "a": 1 }, { "b": 2 }]);
/// let shallow = clone!(&objects);
/// assert_eq!(shallow[0], objects[0]);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(clone!(), json!(null));
/// assert_eq!(clone!(&json!(null)), json!(null));
/// assert_eq!(clone!(&json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! clone {
    () => {
        json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::clone($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::clone($a)
    };
}

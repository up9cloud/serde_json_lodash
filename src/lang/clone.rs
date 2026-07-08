use crate::lib::Value;

/// Fn form of [clone!](crate::clone!); see it for the full docs
///
/// `_x` form: **not provided** — see [clone_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::clone;
/// # use serde_json::json;
/// assert_eq!(clone(&json!(null)), json!(null));
/// ```
pub fn clone(v: &Value) -> Value {
    v.clone()
}

/// See lodash [clone](https://lodash.com/docs/#clone)
///
/// *Note:* JS shallow-copy semantic (sharing references) cannot be expressed
/// with owned `serde_json::Value`, so this is effectively a full copy
///
/// Fn form: [clone()] | `_x` form: **not provided** — see [clone_x()]
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
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(clone!(), json!(null));
/// assert_eq!(clone!(json!(null)), json!(null));
/// assert_eq!(clone!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! clone {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::clone(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::clone(&$a)
    };
}

build_not_provided_x!(clone, clone_x);

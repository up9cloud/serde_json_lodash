use crate::lib::Value;
use crate::get;

/// See lodash [result](https://lodash.com/docs/#result)
///
/// Like [get()], but returns `default` when the resolved value is `null`.
/// Method resolution (invoking functions at the path) is not applicable to
/// JSON
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::result;
/// # use serde_json::json;
/// assert_eq!(result(json!({"a": 1}), json!("b"), json!(5)), json!(5));
/// ```
pub fn result(object: Value, path: Value, default: Value) -> Value {
    match get(object, path, Value::Null) {
        Value::Null => default,
        v => v,
    }
}

/// Based on [result()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": [{ "b": 1 }] });
/// assert_eq!(result!(object.clone(), json!("a[0].b")), json!(1));
/// assert_eq!(result!(object, json!("a[0].c"), json!("default")), json!("default"));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(result!(), json!(null));
/// assert_eq!(result!(json!({"a": 1}), json!("a")), json!(1));
/// assert_eq!(result!(json!({"a": 1}), json!("b"), json!(5)), json!(5));
/// ```
#[macro_export]
macro_rules! result {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::result($a, $b, $crate::lib::json!(null))
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::result($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::result($a, $b, $c)
    };
}

/// `_x` helper for [result()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [result()] and read the returned `Value`.
pub fn result_x() {
    todo!()
}

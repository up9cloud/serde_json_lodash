use crate::lib::{json, Value};

/// `x_`/`_x` helper for [replace()]: takes a primitive argument and returns a primitive value.
pub fn x_replace_x(s: &str, pattern: &str, replacement: &str) -> String {
    s.replacen(pattern, replacement, 1)
}
/// See lodash [replace](https://lodash.com/docs/#replace)
///
/// *Note:* `pattern` is matched as a plain string (like the JS string
/// pattern), regexp patterns are not supported
pub fn replace(v: Value, pattern: Value, replacement: Value) -> Value {
    json!(x_replace_x(
        &crate::to_string_x(v),
        &crate::to_string_x(pattern),
        &crate::to_string_x(replacement),
    ))
}

/// Based on [replace()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   replace!(json!("Hi Fred"), json!("Fred"), json!("Barney")),
///   json!("Hi Barney")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(replace!(), json!(""));
/// assert_eq!(replace!(json!("abc")), json!("abc"));
/// assert_eq!(replace!(json!("abcabc"), json!("b"), json!("x")), json!("axcabc")); // only the first match is replaced
/// assert_eq!(replace!(json!("abc"), json!("z"), json!("x")), json!("abc"));
/// ```
#[macro_export]
macro_rules! replace {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::to_string($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::to_string($a)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::replace($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::replace($a, $b, $c)
    };
}

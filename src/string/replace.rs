use crate::lib::{json, Value};

// internal `&str`/primitive worker for [replace()] / [replace_x()]
fn x_replace_x(s: &str, pattern: &str, replacement: &str) -> String {
    s.replacen(pattern, replacement, 1)
}

/// See lodash [replace](https://lodash.com/docs/#replace)
///
/// *Note:* `pattern` is matched as a plain string (like the JS string
/// pattern), regexp patterns are not supported
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::replace;
/// # use serde_json::json;
/// assert_eq!(replace(json!("Hi Fred"), json!("Fred"), json!("Barney")), json!("Hi Barney"));
/// ```
pub fn replace<A: Into<Value>>(v: A, pattern: Value, replacement: Value) -> Value {
    let v = v.into();
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
/// Additional cases:
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
        $crate::lib::json!("")
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

/// `_x` helper for [replace()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::replace_x;
/// # use serde_json::json;
/// assert_eq!(replace_x(json!("Hi Fred"), json!("Fred"), json!("Barney")), "Hi Barney".to_owned());
/// ```
pub fn replace_x<A: Into<Value>>(v: A, pattern: Value, replacement: Value) -> String {
    let v = v.into();
    x_replace_x(
        &crate::to_string_x(v),
        &crate::to_string_x(pattern),
        &crate::to_string_x(replacement),
    )
}

/// Based on [replace_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(replace_x!(json!("Hi Fred"), json!("Fred"), json!("Barney")), "Hi Barney".to_owned());
/// ```
macro_rules! replace_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::to_string_x($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::to_string_x($a)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::replace_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::replace_x($a, $b, $c)
    };
}

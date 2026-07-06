use crate::lib::{json, Value};

/// `x_` helper for [split()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_split;
/// # use serde_json::json;
/// assert_eq!(x_split("a-b-c", "-", 2), json!(["a", "b"]));
/// ```
pub fn x_split(s: &str, separator: &str, limit: usize) -> Value {
    let parts: Vec<String> = if separator.is_empty() {
        s.chars().take(limit).map(|c| c.to_string()).collect()
    } else {
        s.split(separator)
            .take(limit)
            .map(|p| p.to_owned())
            .collect()
    };
    json!(parts)
}
/// See lodash [split](https://lodash.com/docs/#split)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::split;
/// # use serde_json::json;
/// assert_eq!(split(json!("a-b-c"), "-", 2), json!(["a", "b"]));
/// ```
pub fn split(v: Value, separator: &str, limit: usize) -> Value {
    x_split(&crate::to_string_x(v), separator, limit)
}

/// Based on [split()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   split!(json!("a-b-c"), "-", 2),
///   json!(["a", "b"])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(split!(), json!([]));
/// assert_eq!(split!(json!("a-b-c"), "-"), json!(["a", "b", "c"]));
/// assert_eq!(split!(json!("abc"), ""), json!(["a", "b", "c"]));
/// assert_eq!(split!(json!(null), "-"), json!([""]));
/// ```
#[macro_export]
macro_rules! split {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([$crate::to_string_x($a)])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::split($a, $b, usize::MAX)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::split($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::split($a, $b, $c)
    };
}

/// Based on [x_split()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_split!("a-b-c", "-", 2), json!(["a", "b"]));
/// ```
macro_rules! x_split {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([$a])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_split($a, $b, usize::MAX)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_split($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_split($a, $b, $c)
    };
}

/// `_x` helper for [split()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [split()] and read the returned `Value`.
pub fn split_x() {
    todo!()
}

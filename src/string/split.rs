use crate::lib::{json, Value};

// internal worker for [split()].
fn x_split(s: &str, separator: &str, limit: usize) -> Value {
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
pub fn split<A: Into<Value>>(v: A, separator: &str, limit: usize) -> Value {
    let v = v.into();
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

/// `_x` helper for [split()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [split()] and read the returned `Value`.
pub fn split_x() {
    todo!()
}
/// Based on [split_x()]
#[macro_export]
macro_rules! split_x {
    ($($t:tt)*) => {
        $crate::split_x()
    };
}

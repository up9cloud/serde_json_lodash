use crate::lib::{json, Value};

/// `x_`/`_x` helper for [starts_with()]: takes a primitive argument and returns a primitive value.
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_starts_with_x;
/// assert_eq!(x_starts_with_x("abc", "a", 0), true);
/// ```
pub fn x_starts_with_x(s: &str, target: &str, position: usize) -> bool {
    let tail: String = s.chars().skip(position).collect();
    tail.starts_with(target)
}
/// Based on [x_starts_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// assert_eq!(x_starts_with_x!("abc", "a"), true);
/// ```
#[macro_export]
macro_rules! x_starts_with_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_starts_with_x($a, $b, 0)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_starts_with_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_starts_with_x($a, $b, $c)
    };
}

/// `_x` helper for [starts_with()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::starts_with_x;
/// # use serde_json::json;
/// assert_eq!(starts_with_x(json!("abc"), json!("a"), 0), true);
/// ```
pub fn starts_with_x(v: Value, target: Value, position: usize) -> bool {
    x_starts_with_x(
        &crate::to_string_x(v),
        &crate::to_string_x(target),
        position,
    )
}
/// See lodash [startsWith](https://lodash.com/docs/#startsWith)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::starts_with;
/// # use serde_json::json;
/// assert_eq!(starts_with(json!("abc"), json!("a"), 0), json!(true));
/// ```
pub fn starts_with(v: Value, target: Value, position: usize) -> Value {
    json!(starts_with_x(v, target, position))
}

/// Based on [starts_with_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(starts_with_x!(json!("abc"), json!("a"), 0), true);
/// ```
#[macro_export]
macro_rules! starts_with_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::starts_with_x($a, $b, 0)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::starts_with_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::starts_with_x($a, $b, $c)
    };
}
/// Based on [starts_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(starts_with!(json!("abc"), json!("a")), json!(true));
/// assert_eq!(starts_with!(json!("abc"), json!("b")), json!(false));
/// assert_eq!(starts_with!(json!("abc"), json!("b"), 1), json!(true));
/// assert_eq!(starts_with!(), json!(false));
/// assert_eq!(starts_with!(json!("abc")), json!(false));
/// assert_eq!(starts_with!(json!("abc"), json!("")), json!(true));
/// assert_eq!(starts_with!(json!(null), json!("")), json!(true));
/// ```
#[macro_export]
macro_rules! starts_with {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::starts_with($a, $b, 0)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::starts_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::starts_with($a, $b, $c)
    };
}

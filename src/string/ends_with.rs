use crate::lib::{json, Value};

/// `x_`/`_x` helper for [ends_with()]: takes a primitive argument and returns a primitive value.
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_ends_with_x;
/// assert_eq!(x_ends_with_x("abc", "c", usize::MAX), true);
/// ```
pub fn x_ends_with_x(s: &str, target: &str, position: usize) -> bool {
    let len = s.chars().count();
    let end = position.min(len);
    let head: String = s.chars().take(end).collect();
    head.ends_with(target)
}
/// Based on [x_ends_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// assert_eq!(x_ends_with_x!("abc", "c"), true);
/// ```
#[macro_export]
macro_rules! x_ends_with_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_ends_with_x($a, $b, usize::MAX)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_ends_with_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_ends_with_x($a, $b, $c)
    };
}

/// `_x` helper for [ends_with()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::ends_with_x;
/// # use serde_json::json;
/// assert_eq!(ends_with_x(json!("abc"), json!("c"), usize::MAX), true);
/// ```
pub fn ends_with_x(v: Value, target: Value, position: usize) -> bool {
    x_ends_with_x(
        &crate::to_string_x(v),
        &crate::to_string_x(target),
        position,
    )
}
/// See lodash [endsWith](https://lodash.com/docs/#endsWith)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::ends_with;
/// # use serde_json::json;
/// assert_eq!(ends_with(json!("abc"), json!("c"), usize::MAX), json!(true));
/// ```
pub fn ends_with(v: Value, target: Value, position: usize) -> Value {
    json!(ends_with_x(v, target, position))
}

/// Based on [ends_with_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(ends_with_x!(json!("abc"), json!("c"), usize::MAX), true);
/// ```
#[macro_export]
macro_rules! ends_with_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::ends_with_x($a, $b, usize::MAX)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::ends_with_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::ends_with_x($a, $b, $c)
    };
}
/// Based on [ends_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(ends_with!(json!("abc"), json!("c")), json!(true));
/// assert_eq!(ends_with!(json!("abc"), json!("b")), json!(false));
/// assert_eq!(ends_with!(json!("abc"), json!("b"), 2), json!(true));
/// assert_eq!(ends_with!(), json!(false));
/// assert_eq!(ends_with!(json!("abc")), json!(false));
/// assert_eq!(ends_with!(json!("abc"), json!("")), json!(true));
/// assert_eq!(ends_with!(json!(null), json!("")), json!(true));
/// ```
#[macro_export]
macro_rules! ends_with {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::ends_with($a, $b, usize::MAX)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::ends_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::ends_with($a, $b, $c)
    };
}

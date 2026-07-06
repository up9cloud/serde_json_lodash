use crate::lib::Value;

/// `x_`/`_x` helper for [starts_with()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_starts_with_x;
/// # use serde_json::json;
/// assert_eq!(x_starts_with_x("abc", "b", 1), true);
/// ```
pub fn x_starts_with_x(s: &str, target: &str, position: usize) -> bool {
    let tail: String = s.chars().skip(position).collect();
    tail.starts_with(target)
}
/// See lodash [startsWith](https://lodash.com/docs/#startsWith)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::starts_with;
/// # use serde_json::json;
/// assert_eq!(starts_with(json!("abc"), json!("b"), 1), true);
/// ```
pub fn starts_with(v: Value, target: Value, position: usize) -> bool {
    x_starts_with_x(
        &crate::to_string_x(v),
        &crate::to_string_x(target),
        position,
    )
}

/// Based on [starts_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   starts_with!(json!("abc"), json!("a")),
///   true
/// );
/// assert_eq!(
///   starts_with!(json!("abc"), json!("b")),
///   false
/// );
/// assert_eq!(
///   starts_with!(json!("abc"), json!("b"), 1),
///   true
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(starts_with!(), false);
/// assert_eq!(starts_with!(json!("abc")), false);
/// assert_eq!(starts_with!(json!("abc"), json!("")), true);
/// assert_eq!(starts_with!(json!(null), json!("")), true);
/// ```
#[macro_export]
macro_rules! starts_with {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
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

/// Based on [x_starts_with_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_starts_with_x!("abc", "b", 1), true);
/// ```
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

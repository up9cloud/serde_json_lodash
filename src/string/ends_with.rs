use crate::lib::Value;

/// `x_`/`_x` helper for [ends_with()]: takes a primitive argument and returns a primitive value.
pub fn x_ends_with_x(s: &str, target: &str, position: usize) -> bool {
    let len = s.chars().count();
    let end = position.min(len);
    let head: String = s.chars().take(end).collect();
    head.ends_with(target)
}
/// See lodash [endsWith](https://lodash.com/docs/#endsWith)
pub fn ends_with(v: Value, target: Value, position: usize) -> bool {
    x_ends_with_x(
        &crate::to_string_x(v),
        &crate::to_string_x(target),
        position,
    )
}

/// Based on [ends_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   ends_with!(json!("abc"), json!("c")),
///   true
/// );
/// assert_eq!(
///   ends_with!(json!("abc"), json!("b")),
///   false
/// );
/// assert_eq!(
///   ends_with!(json!("abc"), json!("b"), 2),
///   true
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(ends_with!(), false);
/// assert_eq!(ends_with!(json!("abc")), false);
/// assert_eq!(ends_with!(json!("abc"), json!("")), true);
/// assert_eq!(ends_with!(json!(null), json!("")), true);
/// assert_eq!(serde_json_lodash::x_ends_with_x("abc", "c", usize::MAX), true);
/// ```
#[macro_export]
macro_rules! ends_with {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
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

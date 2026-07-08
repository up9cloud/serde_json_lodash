use crate::lib::{Value, json};

// internal worker for [starts_with()].
fn x_starts_with_x(s: &str, target: &str, position: usize) -> bool {
    let tail: String = s.chars().skip(position).collect();
    tail.starts_with(target)
}

/// Fn form of [starts_with!](crate::starts_with!); see it for the full docs
///
/// `_x` forms: [starts_with_x!](crate::starts_with_x!), [starts_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::starts_with;
/// # use serde_json::json;
/// assert_eq!(starts_with(json!("abc"), json!("a"), 0), json!(true));
/// ```
pub fn starts_with<A: Into<Value>>(v: A, target: Value, position: usize) -> Value {
    let v = v.into();
    json!(starts_with_x(v, target, position))
}

/// See lodash [startsWith](https://lodash.com/docs/#startsWith)
///
/// Fn form: [starts_with()] | `_x` forms: [starts_with_x!](crate::starts_with_x!), [starts_with_x()]
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

/// `_x` helper for [starts_with!](crate::starts_with!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [starts_with_x!](crate::starts_with_x!) | `Value` forms: [starts_with!](crate::starts_with!), [starts_with()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::starts_with_x;
/// # use serde_json::json;
/// assert_eq!(starts_with_x(json!("abc"), json!("a"), 0), true);
/// ```
pub fn starts_with_x<A: Into<Value>>(v: A, target: Value, position: usize) -> bool {
    let v = v.into();
    x_starts_with_x(
        &crate::to_string_x(v),
        &crate::to_string_x(target),
        position,
    )
}

/// `_x` helper for [starts_with!](crate::starts_with!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [starts_with_x()] | `Value` forms: [starts_with!](crate::starts_with!), [starts_with()]
///
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

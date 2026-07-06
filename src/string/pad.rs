use crate::lib::{json, Value};

pub(crate) fn make_padding(n: usize, chars: &str) -> String {
    chars.chars().cycle().take(n).collect()
}

/// `x_`/`_x` helper for [pad()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_pad_x;
/// # use serde_json::json;
/// assert_eq!(x_pad_x("abc", 8, "_-"), "_-abc_-_".to_owned());
/// ```
pub fn x_pad_x(s: &str, length: usize, chars: &str) -> String {
    let len = s.chars().count();
    if length <= len || chars.is_empty() {
        return s.into();
    }
    let total = length - len;
    let start = total / 2;
    let mut out = make_padding(start, chars);
    out.push_str(s);
    out.push_str(&make_padding(total - start, chars));
    out
}
/// See lodash [pad](https://lodash.com/docs/#pad)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pad;
/// # use serde_json::json;
/// assert_eq!(pad(json!("abc"), 8, "_-"), json!("_-abc_-_"));
/// ```
pub fn pad(v: Value, length: usize, chars: &str) -> Value {
    json!(x_pad_x(&crate::to_string_x(v), length, chars))
}

/// Based on [pad()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   pad!(json!("abc"), 8),
///   json!("  abc   ")
/// );
/// assert_eq!(
///   pad!(json!("abc"), 8, "_-"),
///   json!("_-abc_-_")
/// );
/// assert_eq!(
///   pad!(json!("abc"), 3),
///   json!("abc")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pad!(), json!(""));
/// assert_eq!(pad!(json!("abc")), json!("abc"));
/// assert_eq!(pad!(json!(null), 4), json!("    "));
/// assert_eq!(pad!(json!("abc"), 8, ""), json!("abc"));
/// ```
#[macro_export]
macro_rules! pad {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::pad($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pad($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pad($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pad($a, $b, $c)
    };
}

/// `x_` helper for [pad()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_pad;
/// # use serde_json::json;
/// assert_eq!(x_pad("abc", 8, "_-"), json!("_-abc_-_"));
/// ```
pub fn x_pad(s: &str, length: usize, chars: &str) -> Value {
    json!(x_pad_x(s, length, chars))
}
/// `_x` helper for [pad()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pad_x;
/// # use serde_json::json;
/// assert_eq!(pad_x(json!("abc"), 8, "_-"), "_-abc_-_".to_owned());
/// ```
pub fn pad_x(v: Value, length: usize, chars: &str) -> String {
    x_pad_x(&crate::to_string_x(v), length, chars)
}

/// Based on [x_pad_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_pad_x!("abc", 8, "_-"), "_-abc_-_".to_owned());
/// ```
macro_rules! x_pad_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_pad_x($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_pad_x($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_pad_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_pad_x($a, $b, $c)
    };
}
/// Based on [x_pad()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_pad!("abc", 8, "_-"), json!("_-abc_-_"));
/// ```
macro_rules! x_pad {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_pad($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_pad($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_pad($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_pad($a, $b, $c)
    };
}
/// Based on [pad_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pad_x!(json!("abc"), 8, "_-"), "_-abc_-_".to_owned());
/// ```
macro_rules! pad_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::pad_x($a, 0, " ")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pad_x($a, $b, " ")
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pad_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pad_x($a, $b, $c)
    };
}

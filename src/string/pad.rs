use crate::lib::{json, Value};

pub(crate) fn make_padding(n: usize, chars: &str) -> String {
    chars.chars().cycle().take(n).collect()
}

/// `x_`/`_x` helper for [pad()]: takes a primitive argument and returns a primitive value.
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
/// More examples:
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
        json!("")
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

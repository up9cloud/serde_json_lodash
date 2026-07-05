use crate::lib::{json, Value};

/// `x_`/`_x` helper for [trim()]: takes a primitive argument and returns a primitive value.
pub fn x_trim_x(s: &str, chars: &str) -> String {
    if chars.is_empty() {
        return s.into();
    }
    s.trim_matches(|c| chars.contains(c)).into()
}
/// See lodash [trim](https://lodash.com/docs/#trim)
pub fn trim(v: Value, chars: &str) -> Value {
    json!(x_trim_x(&crate::to_string_x(v), chars))
}

/// Based on [trim()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   trim!(json!("  abc  ")),
///   json!("abc")
/// );
/// assert_eq!(
///   trim!(json!("-_-abc-_-"), "_-"),
///   json!("abc")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim!(), json!(""));
/// assert_eq!(trim!(json!(null)), json!(""));
/// assert_eq!(trim!(json!("  abc  "), ""), json!("  abc  "));
/// ```
#[macro_export]
macro_rules! trim {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::trim($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::trim($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::trim($a, $b)
    };
}

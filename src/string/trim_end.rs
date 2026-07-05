use crate::lib::{json, Value};

/// `x_`/`_x` helper for [trim_end()]: takes a primitive argument and returns a primitive value.
pub fn x_trim_end_x(s: &str, chars: &str) -> String {
    if chars.is_empty() {
        return s.into();
    }
    s.trim_end_matches(|c| chars.contains(c)).into()
}
/// See lodash [trimEnd](https://lodash.com/docs/#trimEnd)
pub fn trim_end(v: Value, chars: &str) -> Value {
    json!(x_trim_end_x(&crate::to_string_x(v), chars))
}

/// Based on [trim_end()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   trim_end!(json!("  abc  ")),
///   json!("  abc")
/// );
/// assert_eq!(
///   trim_end!(json!("-_-abc-_-"), "_-"),
///   json!("-_-abc")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim_end!(), json!(""));
/// assert_eq!(trim_end!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! trim_end {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::trim_end($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::trim_end($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::trim_end($a, $b)
    };
}

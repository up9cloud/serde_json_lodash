use crate::lib::{json, Value};

/// `x_`/`_x` helper for [trim_start()]: takes a primitive argument and returns a primitive value.
pub fn x_trim_start_x(s: &str, chars: &str) -> String {
    if chars.is_empty() {
        return s.into();
    }
    s.trim_start_matches(|c| chars.contains(c)).into()
}
/// See lodash [trimStart](https://lodash.com/docs/#trimStart)
pub fn trim_start(v: Value, chars: &str) -> Value {
    json!(x_trim_start_x(&crate::to_string_x(v), chars))
}

/// Based on [trim_start()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   trim_start!(json!("  abc  ")),
///   json!("abc  ")
/// );
/// assert_eq!(
///   trim_start!(json!("-_-abc-_-"), "_-"),
///   json!("abc-_-")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(trim_start!(), json!(""));
/// assert_eq!(trim_start!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! trim_start {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::trim_start($a, " \t\n\r\u{b}\u{c}\u{a0}\u{feff}")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::trim_start($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::trim_start($a, $b)
    };
}

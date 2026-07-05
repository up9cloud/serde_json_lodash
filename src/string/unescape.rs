use crate::lib::{json, Value};

/// `x_`/`_x` helper for [unescape()]: takes a primitive argument and returns a primitive value.
pub fn x_unescape_x(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}
/// `x_` helper for [unescape()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_unescape(s: &str) -> Value {
    json!(x_unescape_x(s))
}
/// `_x` helper for [unescape()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn unescape_x(v: Value) -> String {
    x_unescape_x(&crate::to_string_x(v))
}
/// See lodash [unescape](https://lodash.com/docs/#unescape)
pub fn unescape(v: Value) -> Value {
    json!(unescape_x(v))
}

/// Based on [unescape()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   unescape!(json!("fred, barney, &amp; pebbles")),
///   json!("fred, barney, & pebbles")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(unescape!(), json!(""));
/// assert_eq!(unescape!(json!(null)), json!(""));
/// assert_eq!(unescape!(json!("&lt;b&gt;&quot;quote&quot;&lt;/b&gt; &#39;n&#39; more")), json!("<b>\"quote\"</b> 'n' more"));
/// ```
#[macro_export]
macro_rules! unescape {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::unescape($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::unescape($a)
    };
}

use crate::lib::{json, Value};

/// `x_`/`_x` helper for [escape()]: takes a primitive argument and returns a primitive value.
pub fn x_escape_x(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}
/// `x_` helper for [escape()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_escape(s: &str) -> Value {
    json!(x_escape_x(s))
}
/// `_x` helper for [escape()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn escape_x(v: Value) -> String {
    x_escape_x(&crate::to_string_x(v))
}
/// See lodash [escape](https://lodash.com/docs/#escape)
pub fn escape(v: Value) -> Value {
    json!(escape_x(v))
}

/// Based on [escape()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   escape!(json!("fred, barney, & pebbles")),
///   json!("fred, barney, &amp; pebbles")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(escape!(), json!(""));
/// assert_eq!(escape!(json!(null)), json!(""));
/// assert_eq!(escape!(json!("<b>\"quote\"</b> 'n' more")), json!("&lt;b&gt;&quot;quote&quot;&lt;/b&gt; &#39;n&#39; more"));
/// ```
#[macro_export]
macro_rules! escape {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::escape($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::escape($a)
    };
}

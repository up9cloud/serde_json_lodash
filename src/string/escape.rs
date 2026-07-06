use crate::lib::{json, Value};

/// `x_`/`_x` helper for [escape()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_escape_x;
/// # use serde_json::json;
/// assert_eq!(x_escape_x("fred, barney, & pebbles"), "fred, barney, &amp; pebbles".to_owned());
/// ```
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
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_escape;
/// # use serde_json::json;
/// assert_eq!(x_escape("fred, barney, & pebbles"), json!("fred, barney, &amp; pebbles"));
/// ```
pub fn x_escape(s: &str) -> Value {
    json!(x_escape_x(s))
}
/// `_x` helper for [escape()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::escape_x;
/// # use serde_json::json;
/// assert_eq!(escape_x(json!("fred, barney, & pebbles")), "fred, barney, &amp; pebbles".to_owned());
/// ```
pub fn escape_x(v: Value) -> String {
    x_escape_x(&crate::to_string_x(v))
}
/// See lodash [escape](https://lodash.com/docs/#escape)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::escape;
/// # use serde_json::json;
/// assert_eq!(escape(json!("fred, barney, & pebbles")), json!("fred, barney, &amp; pebbles"));
/// ```
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
/// Additional cases:
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
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::escape($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::escape($a)
    };
}

/// Based on [x_escape_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_escape_x!("fred, barney, & pebbles"), "fred, barney, &amp; pebbles".to_owned());
/// ```
macro_rules! x_escape_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_escape_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_escape_x($a)
    };
}
/// Based on [x_escape()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_escape!("fred, barney, & pebbles"), json!("fred, barney, &amp; pebbles"));
/// ```
macro_rules! x_escape {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_escape($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_escape($a)
    };
}
/// Based on [escape_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(escape_x!(json!("fred, barney, & pebbles")), "fred, barney, &amp; pebbles".to_owned());
/// ```
macro_rules! escape_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::escape_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::escape_x($a)
    };
}

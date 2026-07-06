use crate::lib::{json, Value};

// internal `&str`/primitive worker for [unescape()] / [unescape_x()]
fn x_unescape_x(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

/// `_x` helper for [unescape()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::unescape_x;
/// # use serde_json::json;
/// assert_eq!(unescape_x(json!("fred, barney, &amp; pebbles")), "fred, barney, & pebbles".to_owned());
/// ```
pub fn unescape_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_unescape_x(&crate::to_string_x(v))
}

/// See lodash [unescape](https://lodash.com/docs/#unescape)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::unescape;
/// # use serde_json::json;
/// assert_eq!(unescape(json!("fred, barney, &amp; pebbles")), json!("fred, barney, & pebbles"));
/// ```
pub fn unescape<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
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
/// Additional cases:
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
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::unescape($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::unescape($a)
    };
}

/// Based on [unescape_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(unescape_x!(json!("fred, barney, &amp; pebbles")), "fred, barney, & pebbles".to_owned());
/// ```
macro_rules! unescape_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::unescape_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::unescape_x($a)
    };
}

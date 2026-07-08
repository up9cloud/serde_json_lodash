use crate::lib::{Value, json};

// internal `&str`/primitive worker for [escape()] / [escape_x()]
fn x_escape_x(s: &str) -> String {
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

/// Fn form of [escape!](crate::escape!); see it for the full docs
///
/// `_x` forms: [escape_x!](crate::escape_x!), [escape_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::escape;
/// # use serde_json::json;
/// assert_eq!(escape(json!("fred, barney, & pebbles")), json!("fred, barney, &amp; pebbles"));
/// ```
pub fn escape<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    json!(escape_x(v))
}

/// See lodash [escape](https://lodash.com/docs/#escape)
///
/// Fn form: [escape()] | `_x` forms: [escape_x!](crate::escape_x!), [escape_x()]
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

/// `_x` helper for [escape!](crate::escape!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [escape_x!](crate::escape_x!) | `Value` forms: [escape!](crate::escape!), [escape()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::escape_x;
/// # use serde_json::json;
/// assert_eq!(escape_x(json!("fred, barney, & pebbles")), "fred, barney, &amp; pebbles".to_owned());
/// ```
pub fn escape_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_escape_x(&crate::to_string_x(v))
}

/// `_x` helper for [escape!](crate::escape!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [escape_x()] | `Value` forms: [escape!](crate::escape!), [escape()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(escape_x!(json!("fred, barney, & pebbles")), "fred, barney, &amp; pebbles".to_owned());
/// ```
#[macro_export]
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

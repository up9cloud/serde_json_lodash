use crate::lib::{json, Value};

// internal `&str`/primitive worker for [truncate()] / [truncate_x()]
fn x_truncate_x(s: &str, options: &Value) -> String {
    let length = options["length"].as_u64().unwrap_or(30) as usize;
    let omission = options["omission"].as_str().unwrap_or("...");
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= length {
        return s.into();
    }
    let end = length.saturating_sub(omission.chars().count());
    let mut result: String = chars[..end].iter().collect();
    if let Some(sep) = options["separator"].as_str().filter(|sep| !sep.is_empty())
        && let Some(idx) = result.rfind(sep)
    {
        result.truncate(idx);
    }
    result.push_str(omission);
    result
}

/// See lodash [truncate](https://lodash.com/docs/#truncate)
///
/// `options` is an object like `json!({"length": 24, "omission": "...",
/// "separator": " "})`; regexp separators are not supported
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::truncate;
/// # use serde_json::json;
/// assert_eq!(truncate(json!("hi-diddly-ho there, neighborino"), json!({"length": 24, "separator": " "})), json!("hi-diddly-ho there,..."));
/// ```
pub fn truncate<A: Into<Value>>(v: A, options: Value) -> Value {
    let v = v.into();
    json!(x_truncate_x(&crate::to_string_x(v), &options))
}

/// Based on [truncate()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   truncate!(json!("hi-diddly-ho there, neighborino")),
///   json!("hi-diddly-ho there, neighbo...")
/// );
/// assert_eq!(
///   truncate!(json!("hi-diddly-ho there, neighborino"), json!({"length": 24, "separator": " "})),
///   json!("hi-diddly-ho there,...")
/// );
/// assert_eq!(
///   truncate!(json!("hi-diddly-ho there, neighborino"), json!({"omission": " [...]"})),
///   json!("hi-diddly-ho there, neig [...]")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(truncate!(), json!(""));
/// assert_eq!(truncate!(json!("short")), json!("short"));
/// assert_eq!(truncate!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! truncate {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::truncate($a, $crate::lib::json!({}))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::truncate($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::truncate($a, $b)
    };
}

/// `_x` helper for [truncate()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::truncate_x;
/// # use serde_json::json;
/// assert_eq!(truncate_x(json!("hi-diddly-ho there, neighborino"), json!({"length": 24, "separator": " "})), "hi-diddly-ho there,...".to_owned());
/// ```
pub fn truncate_x<A: Into<Value>>(v: A, options: Value) -> String {
    let v = v.into();
    x_truncate_x(&crate::to_string_x(v), &options)
}

/// Based on [truncate_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(truncate_x!(json!("hi-diddly-ho there, neighborino"), json!({"length": 24, "separator": " "})), "hi-diddly-ho there,...".to_owned());
/// ```
macro_rules! truncate_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::truncate_x($a, $crate::lib::json!({}))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::truncate_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::truncate_x($a, $b)
    };
}

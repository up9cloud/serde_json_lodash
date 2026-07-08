use crate::lib::{Value, json};

// internal `&str`/primitive worker for [lower_first()] / [lower_first_x()]
fn x_lower_first_x(s: &str) -> String {
    let mut cs = s.chars();
    match cs.next() {
        Some(c) => {
            let mut out = c.to_lowercase().to_string();
            out.push_str(cs.as_str());
            out
        }
        None => String::new(),
    }
}

/// Fn form of [lower_first!](crate::lower_first!); see it for the full docs
///
/// `_x` forms: [lower_first_x!](crate::lower_first_x!), [lower_first_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::lower_first;
/// # use serde_json::json;
/// assert_eq!(lower_first(json!("Fred")), json!("fred"));
/// ```
pub fn lower_first<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    json!(lower_first_x(v))
}

/// See lodash [lowerFirst](https://lodash.com/docs/#lowerFirst)
///
/// Fn form: [lower_first()] | `_x` forms: [lower_first_x!](crate::lower_first_x!), [lower_first_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   lower_first!(json!("Fred")),
///   json!("fred")
/// );
/// assert_eq!(
///   lower_first!(json!("FRED")),
///   json!("fRED")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lower_first!(), json!(""));
/// assert_eq!(lower_first!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! lower_first {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::lower_first($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::lower_first($a)
    };
}

/// `_x` helper for [lower_first!](crate::lower_first!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [lower_first_x!](crate::lower_first_x!) | `Value` forms: [lower_first!](crate::lower_first!), [lower_first()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::lower_first_x;
/// # use serde_json::json;
/// assert_eq!(lower_first_x(json!("Fred")), "fred".to_owned());
/// ```
pub fn lower_first_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_lower_first_x(&crate::to_string_x(v))
}

/// `_x` helper for [lower_first!](crate::lower_first!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [lower_first_x()] | `Value` forms: [lower_first!](crate::lower_first!), [lower_first()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lower_first_x!(json!("Fred")), "fred".to_owned());
/// ```
#[macro_export]
macro_rules! lower_first_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::lower_first_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::lower_first_x($a)
    };
}

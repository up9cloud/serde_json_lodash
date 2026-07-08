use crate::lib::{Value, json};

// internal `&str`/primitive worker for [escape_reg_exp()] / [escape_reg_exp_x()]
fn x_escape_reg_exp_x(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if matches!(
            c,
            '^' | '$' | '\\' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|'
        ) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Fn form of [escape_reg_exp!](crate::escape_reg_exp!); see it for the full docs
///
/// `_x` forms: [escape_reg_exp_x!](crate::escape_reg_exp_x!), [escape_reg_exp_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::escape_reg_exp;
/// # use serde_json::json;
/// assert_eq!(escape_reg_exp(json!("[lodash](https://lodash.com/)")), json!("\\[lodash\\]\\(https://lodash\\.com/\\)"));
/// ```
pub fn escape_reg_exp<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    json!(escape_reg_exp_x(v))
}

/// See lodash [escapeRegExp](https://lodash.com/docs/#escapeRegExp)
///
/// Fn form: [escape_reg_exp()] | `_x` forms: [escape_reg_exp_x!](crate::escape_reg_exp_x!), [escape_reg_exp_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   escape_reg_exp!(json!("[lodash](https://lodash.com/)")),
///   json!("\\[lodash\\]\\(https://lodash\\.com/\\)")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(escape_reg_exp!(), json!(""));
/// assert_eq!(escape_reg_exp!(json!(null)), json!(""));
/// assert_eq!(escape_reg_exp!(json!("a|b")), json!("a\\|b"));
/// ```
#[macro_export]
macro_rules! escape_reg_exp {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::escape_reg_exp($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::escape_reg_exp($a)
    };
}

/// `_x` helper for [escape_reg_exp!](crate::escape_reg_exp!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [escape_reg_exp_x!](crate::escape_reg_exp_x!) | `Value` forms: [escape_reg_exp!](crate::escape_reg_exp!), [escape_reg_exp()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::escape_reg_exp_x;
/// # use serde_json::json;
/// assert_eq!(escape_reg_exp_x(json!("[lodash](https://lodash.com/)")), "\\[lodash\\]\\(https://lodash\\.com/\\)".to_owned());
/// ```
pub fn escape_reg_exp_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_escape_reg_exp_x(&crate::to_string_x(v))
}

/// `_x` helper for [escape_reg_exp!](crate::escape_reg_exp!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [escape_reg_exp_x()] | `Value` forms: [escape_reg_exp!](crate::escape_reg_exp!), [escape_reg_exp()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(escape_reg_exp_x!(json!("[lodash](https://lodash.com/)")), "\\[lodash\\]\\(https://lodash\\.com/\\)".to_owned());
/// ```
#[macro_export]
macro_rules! escape_reg_exp_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::escape_reg_exp_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::escape_reg_exp_x($a)
    };
}

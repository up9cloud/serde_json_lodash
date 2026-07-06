use crate::lib::{json, Value};

/// `x_`/`_x` helper for [escape_reg_exp()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_escape_reg_exp_x;
/// # use serde_json::json;
/// assert_eq!(x_escape_reg_exp_x("[lodash](https://lodash.com/)"), "\\[lodash\\]\\(https://lodash\\.com/\\)".to_owned());
/// ```
pub fn x_escape_reg_exp_x(s: &str) -> String {
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
/// `x_` helper for [escape_reg_exp()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_escape_reg_exp;
/// # use serde_json::json;
/// assert_eq!(x_escape_reg_exp("[lodash](https://lodash.com/)"), json!("\\[lodash\\]\\(https://lodash\\.com/\\)"));
/// ```
pub fn x_escape_reg_exp(s: &str) -> Value {
    json!(x_escape_reg_exp_x(s))
}
/// `_x` helper for [escape_reg_exp()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::escape_reg_exp_x;
/// # use serde_json::json;
/// assert_eq!(escape_reg_exp_x(json!("[lodash](https://lodash.com/)")), "\\[lodash\\]\\(https://lodash\\.com/\\)".to_owned());
/// ```
pub fn escape_reg_exp_x(v: Value) -> String {
    x_escape_reg_exp_x(&crate::to_string_x(v))
}
/// See lodash [escapeRegExp](https://lodash.com/docs/#escapeRegExp)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::escape_reg_exp;
/// # use serde_json::json;
/// assert_eq!(escape_reg_exp(json!("[lodash](https://lodash.com/)")), json!("\\[lodash\\]\\(https://lodash\\.com/\\)"));
/// ```
pub fn escape_reg_exp(v: Value) -> Value {
    json!(escape_reg_exp_x(v))
}

/// Based on [escape_reg_exp()]
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

/// Based on [x_escape_reg_exp_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_escape_reg_exp_x!("[lodash](https://lodash.com/)"), "\\[lodash\\]\\(https://lodash\\.com/\\)".to_owned());
/// ```
macro_rules! x_escape_reg_exp_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_escape_reg_exp_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_escape_reg_exp_x($a)
    };
}
/// Based on [x_escape_reg_exp()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_escape_reg_exp!("[lodash](https://lodash.com/)"), json!("\\[lodash\\]\\(https://lodash\\.com/\\)"));
/// ```
macro_rules! x_escape_reg_exp {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_escape_reg_exp($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_escape_reg_exp($a)
    };
}
/// Based on [escape_reg_exp_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(escape_reg_exp_x!(json!("[lodash](https://lodash.com/)")), "\\[lodash\\]\\(https://lodash\\.com/\\)".to_owned());
/// ```
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

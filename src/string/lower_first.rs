use crate::lib::{json, Value};

/// `x_`/`_x` helper for [lower_first()]: takes a primitive argument and returns a primitive value.
pub fn x_lower_first_x(s: &str) -> String {
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
/// `x_` helper for [lower_first()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_lower_first(s: &str) -> Value {
    json!(x_lower_first_x(s))
}
/// `_x` helper for [lower_first()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn lower_first_x(v: Value) -> String {
    x_lower_first_x(&crate::to_string_x(v))
}
/// See lodash [lowerFirst](https://lodash.com/docs/#lowerFirst)
pub fn lower_first(v: Value) -> Value {
    json!(lower_first_x(v))
}

/// Based on [lower_first()]
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
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lower_first!(), json!(""));
/// assert_eq!(lower_first!(json!(null)), json!(""));
/// assert_eq!(serde_json_lodash::x_lower_first_x(""), "".to_owned());
/// ```
#[macro_export]
macro_rules! lower_first {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::lower_first($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::lower_first($a)
    };
}

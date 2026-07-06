use crate::lib::{json, Value};

// internal `&str`/primitive worker for [to_upper()] / [to_upper_x()]
fn x_to_upper_x(s: &str) -> String {
    s.to_uppercase()
}

/// `_x` helper for [to_upper()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_upper_x;
/// # use serde_json::json;
/// assert_eq!(to_upper_x(json!("--foo-bar--")), "--FOO-BAR--".to_owned());
/// ```
pub fn to_upper_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    x_to_upper_x(&crate::to_string_x(v))
}

/// See lodash [toUpper](https://lodash.com/docs/#toUpper)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_upper;
/// # use serde_json::json;
/// assert_eq!(to_upper(json!("--foo-bar--")), json!("--FOO-BAR--"));
/// ```
pub fn to_upper<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    json!(to_upper_x(v))
}

/// Based on [to_upper()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   to_upper!(json!("--foo-bar--")),
///   json!("--FOO-BAR--")
/// );
/// assert_eq!(
///   to_upper!(json!("fooBar")),
///   json!("FOOBAR")
/// );
/// assert_eq!(
///   to_upper!(json!("__foo_bar__")),
///   json!("__FOO_BAR__")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_upper!(), json!(""));
/// assert_eq!(to_upper!(json!(null)), json!(""));
/// assert_eq!(to_upper!(json!([1, 2, 3])), json!("1,2,3"));
/// ```
#[macro_export]
macro_rules! to_upper {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::to_upper($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_upper($a)
    };
}

/// Based on [to_upper_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_upper_x!(json!("--foo-bar--")), "--FOO-BAR--".to_owned());
/// ```
macro_rules! to_upper_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::to_upper_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_upper_x($a)
    };
}

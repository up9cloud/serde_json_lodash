use crate::lib::Value;

use crate::internal::type_name;

// internal `&str`/primitive worker for [to_lower()] / [to_lower_x()]
fn x_to_lower_x(s: &str) -> String {
    s.to_lowercase()
}

/// Fn form of [to_lower!](crate::to_lower!); see it for the full docs
///
/// `_x` forms: [to_lower_x!](crate::to_lower_x!), [to_lower_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_lower;
/// # use serde_json::json;
/// assert_eq!(to_lower(json!("--Foo-Bar--")), json!("--foo-bar--"));
/// ```
pub fn to_lower<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    Value::String(to_lower_x(v))
}

/// See lodash [toLower](https://lodash.com/docs/#toLower)
///
/// Fn form: [to_lower()] | `_x` forms: [to_lower_x!](crate::to_lower_x!), [to_lower_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   to_lower!(json!("--Foo-Bar--")),
///   json!("--foo-bar--")
/// );
/// assert_eq!(
///   to_lower!(json!("fooBar")),
///   json!("foobar")
/// );
/// assert_eq!(
///   to_lower!(json!("__FOO_BAR__")),
///   json!("__foo_bar__")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_lower!(), json!(""));
/// assert_eq!(to_lower!(json!(null)), json!(""));
/// assert_eq!(to_lower!(json!(false)), json!("false"));
/// assert_eq!(to_lower!(json!(-0)), json!("0")); // rust world -0 is 0
/// assert_eq!(to_lower!(json!("")), json!(""));
/// assert_eq!(to_lower!(json!([])), json!(""));
/// assert_eq!(to_lower!(json!([null,'A',{}])), json!("null,a,serde_json::map::map<alloc::string::string, serde_json::value::value>"));
/// assert_eq!(to_lower!(json!({})), json!("serde_json::map::map<alloc::string::string, serde_json::value::value>"));
/// ```
#[macro_export]
macro_rules! to_lower {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::to_lower($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_lower($a)
    };
}

/// `_x` helper for [to_lower!](crate::to_lower!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [to_lower_x!](crate::to_lower_x!) | `Value` forms: [to_lower!](crate::to_lower!), [to_lower()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_lower_x;
/// # use serde_json::json;
/// assert_eq!(to_lower_x(json!("--Foo-Bar--")), "--foo-bar--".to_owned());
/// ```
pub fn to_lower_x<A: Into<Value>>(v: A) -> String {
    let v = v.into();
    match v {
        Value::Null => "".into(),
        Value::Bool(b) => {
            if b {
                "true".into()
            } else {
                "false".into()
            }
        }
        Value::String(s) => x_to_lower_x(&s),
        Value::Number(n) => n.to_string(),
        Value::Array(vec) => {
            let mut result = vec![];
            for item in vec.into_iter() {
                if item.is_null() {
                    result.push("null".into())
                } else {
                    result.push(to_lower_x(item));
                }
            }
            result.join(",")
        }
        Value::Object(o) => x_to_lower_x(type_name(&o)),
    }
}

/// `_x` helper for [to_lower!](crate::to_lower!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [to_lower_x()] | `Value` forms: [to_lower!](crate::to_lower!), [to_lower()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_lower_x!(json!("--Foo-Bar--")), "--foo-bar--".to_owned());
/// ```
#[macro_export]
macro_rules! to_lower_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::to_lower_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_lower_x($a)
    };
}

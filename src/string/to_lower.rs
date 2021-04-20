use crate::lib::{json, Value, get_type_name};

///
pub fn x_to_lower_x(s: &str) -> String {
    s.to_lowercase()
}
///
pub fn x_to_lower(s: &str) -> Value {
    json!(x_to_lower_x(s))
}
///
pub fn to_lower_x(v: Value) -> String {
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
        Value::Object(o) => x_to_lower_x(get_type_name(&o)),
    }
}
///
pub fn to_lower(v: Value) -> Value {
    Value::String(to_lower_x(v))
}
/// Description can be found in [lodash toLower](https://lodash.com/docs/#toLower)
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
/// More examples:
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
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::to_lower($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_lower($a)
    };
}

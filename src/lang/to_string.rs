use crate::lib::{json, Value, get_type_name};

///
pub fn x_to_string(v: &str) -> Value {
    json!(v)
}
///
pub fn to_string_x(v: Value) -> String {
    match v {
        Value::Null => "".into(),
        Value::Bool(b) => {
            if b {
                "true".into()
            } else {
                "false".into()
            }
        }
        Value::Number(n) => n.to_string(),
        Value::String(s) => s,
        Value::Array(vec) => {
            let mut iter = vec.into_iter();
            match iter.next() {
                Some(v) => {
                    let mut s = "".to_owned();
                    if v.is_null() {
                        s.push_str("null");
                    } else {
                        s.push_str(&*to_string_x(v));
                    }
                    for v in iter {
                        s.push(',');
                        if v.is_null() {
                            s.push_str("null");
                        } else {
                            s.push_str(&*to_string_x(v));
                        }
                    }
                    s
                }
                None => "".into(),
            }
        }
        Value::Object(o) => get_type_name(&o).into(), // I don't think put [object Object] here is a good idea, so...
    }
}
/// See lodash [toString](https://lodash.com/docs/#toString)
pub fn to_string(v: Value) -> Value {
    Value::String(to_string_x(v))
}

/// Based on [x_to_string()]
#[macro_export]
macro_rules! x_to_string {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_to_string($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_to_string($a)
    };
}
/// Based on [to_string_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   to_string_x!(json!(null)),
///   "".to_owned()
/// );
/// assert_eq!(
///   to_string_x!(json!(-0)),
///   "0".to_owned() // In rust world, -0 is 0
/// );
/// assert_eq!(
///   to_string_x!(json!([1, 2, 3])),
///   "1,2,3".to_owned()
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_string_x!(), "".to_owned());
/// ```
#[macro_export]
macro_rules! to_string_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::to_string_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_string_x($a)
    };
}
/// Based on [to_string()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
///
/// assert_eq!(
///   to_string!(json!(null)),
///   json!("")
/// );
/// assert_eq!(
///   to_string!(json!(-0)),
///   json!("0") // In rust world, -0 is 0
/// );
/// assert_eq!(
///   to_string!(json!([1, 2, 3])),
///   json!("1,2,3")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_string!(), json!(""));
/// assert_eq!(to_string!(json!(null)), json!(""));
/// assert_eq!(to_string!(json!(false)), json!("false"));
/// assert_eq!(to_string!(json!(-0)), json!("0")); // rust world -0 is 0
/// assert_eq!(to_string!(json!("")), json!(""));
/// assert_eq!(to_string!(json!([])), json!(""));
/// assert_eq!(to_string!(json!([null,"A",{}])), json!("null,A,serde_json::map::Map<alloc::string::String, serde_json::value::Value>"));
/// assert_eq!(to_string!(json!({})), json!("serde_json::map::Map<alloc::string::String, serde_json::value::Value>"));
/// ```
#[macro_export]
macro_rules! to_string {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::to_string($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_string($a)
    };
}

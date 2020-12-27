use crate::lib::{json, Value};

///
pub fn x_to_string_x(v: &str) -> &str {
    v
}
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
                    s.push_str(to_string(v).as_str().unwrap_or(""));
                    for v in iter {
                        s.push(',');
                        s.push_str(to_string(v).as_str().unwrap_or(""));
                    }
                    s
                }
                None => "".into(),
            }
        }
        Value::Object(_) => "Map<String, Value>".into(), // I don't think put [object Object] here is a good idea, so...
    }
}
/// See [lodash toString](https://lodash.com/docs/#toString)
///
/// Examples:
///
/// ```rust
/// use serde_json::json;
/// use serde_json_lodash::to_string;
/// assert_eq!(
///   to_string(json!(null)),
///   json!("")
/// );
/// assert_eq!(
///   to_string(json!(-0)),
///   json!("0") // In rust world, -0 is 0
/// );
/// assert_eq!(
///   to_string(json!([1, 2, 3])),
///   json!("1,2,3")
/// );
/// ```
pub fn to_string(v: Value) -> Value {
    match v {
        Value::Null => json!(""),
        Value::Bool(b) => {
            if b {
                json!("true")
            } else {
                json!("false")
            }
        }
        Value::Number(n) => {
            json!(n.to_string())
        }
        Value::String(_) => v,
        Value::Array(vec) => {
            let mut iter = vec.into_iter();
            match iter.next() {
                Some(v) => {
                    let mut s = "".to_owned();
                    s.push_str(to_string(v).as_str().unwrap_or(""));
                    for v in iter {
                        s.push(',');
                        s.push_str(to_string(v).as_str().unwrap_or(""));
                    }
                    json!(s)
                }
                None => json!(""),
            }
        }
        Value::Object(_) => json!("Map<String, Value>"), // I don't think put [object Object] here is a good idea, so...
    }
}
#[doc(hidden)]
pub use to_string as toString;

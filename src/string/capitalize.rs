use crate::lib::{json, Value};

///
pub fn x_capitalize_x(s: &str) -> String {
    if s.is_empty() {
        s.into()
    } else if s.len() == 1 {
        s.to_uppercase()
    } else {
        let mut ss = s.chars().next().unwrap().to_uppercase().to_string();
        ss.push_str(&s[1..].to_lowercase());
        ss
    }
}
///
pub fn x_capitalize(s: &str) -> Value {
    json!(x_capitalize_x(s))
}
///
pub fn capitalize_x(v: Value) -> String {
    match v {
        Value::Null => "".into(),
        Value::Bool(b) => {
            if b {
                "True".into()
            } else {
                "False".into()
            }
        }
        Value::String(s) => x_capitalize_x(&s),
        Value::Number(n) => n.to_string(),
        Value::Array(vec) => {
            let mut iter = vec.into_iter();
            match iter.next() {
                Some(v) => {
                    let mut s = capitalize_x(v);
                    for v in iter {
                        s.push(',');
                        s.push_str(&crate::to_string_x(v));
                    }
                    s
                }
                None => "".into(),
            }
        }
        Value::Object(_) => crate::to_string_x(v),
    }
}
///
pub fn capitalize(v: Value) -> Value {
    match v {
        Value::Null => json!(""),
        Value::Bool(b) => {
            if b {
                json!("True")
            } else {
                json!("False")
            }
        }
        Value::String(s) => x_capitalize(&s),
        Value::Number(n) => {
            json!(n.to_string())
        }
        Value::Array(vec) => {
            let mut iter = vec.into_iter();
            match iter.next() {
                Some(v) => {
                    let mut s = capitalize_x(v);
                    for v in iter {
                        s.push(',');
                        s.push_str(&crate::to_string_x(v));
                    }
                    json!(s)
                }
                None => json!(""),
            }
        }
        Value::Object(_) => {
            json!(crate::to_string_x(v))
        }
    }
}
/// See [lodash capitalize](https://lodash.com/docs/#capitalize)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   capitalize!(json!("FRED")),
///   json!("Fred")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(capitalize!(), json!(""));
/// assert_eq!(capitalize!(json!([])), json!(""));
/// assert_eq!(capitalize!(json!(['a',2])), json!("A,2"));
/// assert_eq!(capitalize!(json!({})), json!("Map<String, Value>"));
/// ```
#[macro_export]
macro_rules! capitalize {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::capitalize($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::capitalize($a)
    };
}

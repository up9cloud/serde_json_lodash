use crate::lib::{json, Value, get_type_name};

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
                    let mut s = {
                        if v.is_null() {
                            "Null".into()
                        } else {
                            capitalize_x(v)
                        }
                    };
                    for v in iter {
                        s.push(',');
                        s.push_str(&crate::to_lower_x(v));
                    }
                    s
                }
                None => "".into(),
            }
        }
        Value::Object(o) => x_capitalize_x(get_type_name(&o)),
    }
}
///
pub fn capitalize(v: Value) -> Value {
    Value::String(capitalize_x(v))
}
/// Description can be found in [lodash capitalize](https://lodash.com/docs/#capitalize)
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
/// assert_eq!(capitalize!(json!(null)), json!(""));
/// assert_eq!(capitalize!(json!(false)), json!("False"));
/// assert_eq!(capitalize!(json!(-0)), json!("0")); // rust world -0 is 0
/// assert_eq!(capitalize!(json!("")), json!(""));
/// assert_eq!(capitalize!(json!([])), json!(""));
/// assert_eq!(capitalize!(json!([null,'A',{}])), json!("Null,a,serde_json::map::map<alloc::string::string, serde_json::value::value>"));
/// assert_eq!(capitalize!(json!({})), json!("Serde_json::map::map<alloc::string::string, serde_json::value::value>"));
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

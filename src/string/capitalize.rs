use crate::lib::Value;

use crate::internal::type_name;

// Private worker: the actual "upper-first, lower-rest" algorithm on a `&str`.
// (Was the public `x_capitalize_x`; now that `capitalize`/`capitalize_x` accept
// `impl Into<Value>`, a `&str` argument reaches them directly, so this only
// needs to exist as an internal helper.)
fn capitalize_str(s: &str) -> String {
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

/// Fn form of [capitalize!](crate::capitalize!); see it for the full docs
///
/// `_x` forms: [capitalize_x!](crate::capitalize_x!), [capitalize_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::capitalize;
/// # use serde_json::json;
/// assert_eq!(capitalize("FRED"), json!("Fred"));
/// assert_eq!(capitalize(json!("FRED")), json!("Fred"));
/// ```
pub fn capitalize<A: Into<Value>>(v: A) -> Value {
    Value::String(capitalize_x(v))
}

/// See lodash [capitalize](https://lodash.com/docs/#capitalize)
///
/// Accepts anything convertible into a `Value` — a `&str`/`String` primitive or a `json!` value.
///
/// Fn form: [capitalize()] | `_x` forms: [capitalize_x!](crate::capitalize_x!), [capitalize_x()]
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
/// // a primitive `&str` argument is accepted too
/// assert_eq!(capitalize!("FRED"), json!("Fred"));
/// ```
///
/// Additional cases:
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
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::capitalize($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::capitalize($a)
    };
}

/// `_x` helper for [capitalize!](crate::capitalize!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Accepts anything convertible into a `Value` — a `&str`/`String` primitive or a `json!` value.
///
/// Macro form: [capitalize_x!](crate::capitalize_x!) | `Value` forms: [capitalize!](crate::capitalize!), [capitalize()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::capitalize_x;
/// # use serde_json::json;
/// assert_eq!(capitalize_x("FRED"), "Fred".to_owned());
/// assert_eq!(capitalize_x(json!("FRED")), "Fred".to_owned());
/// ```
pub fn capitalize_x<A: Into<Value>>(v: A) -> String {
    match v.into() {
        Value::Null => "".into(),
        Value::Bool(b) => {
            if b {
                "True".into()
            } else {
                "False".into()
            }
        }
        Value::String(s) => capitalize_str(&s),
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
        Value::Object(o) => capitalize_str(type_name(&o)),
    }
}

/// `_x` helper for [capitalize!](crate::capitalize!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [capitalize_x()] | `Value` forms: [capitalize!](crate::capitalize!), [capitalize()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(capitalize_x!("FRED"), "Fred".to_owned());
/// assert_eq!(capitalize_x!(json!("FRED")), "Fred".to_owned());
/// ```
#[macro_export]
macro_rules! capitalize_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::capitalize_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::capitalize_x($a)
    };
}

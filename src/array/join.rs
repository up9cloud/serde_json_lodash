use crate::lib::{Value};

///
pub fn join_x(v: Value, sep: &str) -> String {
    match v {
        Value::Null => "".into(),
        Value::Bool(_) => "".into(),
        Value::Number(_) => "".into(),
        Value::String(s) => {
            if s.is_empty() {
                return "".into();
            }
            s.chars()
                .map(|c| c.into())
                .collect::<Vec<String>>()
                .join(sep)
        }
        Value::Array(vec) => {
            let mut result = vec![];
            for item in vec.into_iter() {
                result.push(crate::to_string_x(item));
            }
            result.join(sep)
        }
        Value::Object(_) => "".into(),
    }
}

/// See lodash [join](https://lodash.com/docs/#join)
pub fn join(v: Value, sep: &str) -> Value {
    Value::String(join_x(v, sep))
}

/// Based on [join_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   join_x!(json!(['a', 'b', 'c']), "~"),
///   "a~b~c".to_owned()
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(join!(), "".to_owned());
/// ```
#[macro_export]
macro_rules! join_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::join_x($a, ",")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::join_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::join_x($a, $b)
    };
}
/// Based on [join()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   join!(json!(['a', 'b', 'c']), "~"),
///   json!("a~b~c")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(join!(), json!(""));
/// assert_eq!(join!(json!(null)), json!(""));
/// assert_eq!(join!(json!(false)), json!(""));
/// assert_eq!(join!(json!(0)), json!(""));
/// assert_eq!(join!(json!("")), json!(""));
/// assert_eq!(join!(json!("ab")), json!("a,b"));
/// assert_eq!(join!(json!("初音"), "🥰"), json!("初🥰音"));
/// assert_eq!(join!(json!([])), json!(""));
/// assert_eq!(join!(json!([{},[],[1,[2,[3]]]])), json!("serde_json::map::Map<alloc::string::String, serde_json::value::Value>,,1,2,3"));
/// assert_eq!(join!(json!({})), json!(""));
/// ```
#[macro_export]
macro_rules! join {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::join($a, ",")
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::join($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::join($a, $b)
    };
}

use crate::lib::{json, Value};
use crate::{to_string, to_string_x};

///
pub fn x_to_path_x(s: &str) -> Vec<String> {
    let mut vec = vec![];
    let mut prev = "".to_owned();
    let mut current = "".to_owned();
    let mut in_square = false;
    for c in s.chars() {
        match c {
            '.' => {
                if in_square {
                    current.push_str(&c.to_string());
                } else if prev != "]" {
                    vec.push(current);
                    current = "".to_owned();
                }
            }
            '[' => {
                in_square = true;
                if !current.is_empty() {
                    vec.push(current);
                    current = "".to_owned();
                }
            }
            ']' => {
                in_square = false;
                if prev == "]" {
                    break;
                }
                let len = current.len();
                if len >= 2 {
                    let mut cs = current.chars();
                    let first = cs.next().unwrap();
                    let last = cs.last().unwrap();
                    if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
                        current = current[1..(len - 1)].to_owned();
                    }
                }
                vec.push(current);
                current = "".to_owned();
            }
            _ => {
                current.push_str(&c.to_string());
            }
        }
        prev = c.to_string();
    }
    if prev == "." || !current.is_empty() {
        vec.push(current);
    }
    vec
}
///
pub fn x_to_path(s: &str) -> Value {
    let mut vec = vec![];
    let mut prev = "".to_owned();
    let mut current = "".to_owned();
    let mut in_square = false;
    for c in s.chars() {
        match c {
            '.' => {
                if in_square {
                    current.push_str(&c.to_string());
                } else if prev != "]" {
                    vec.push(json!(current));
                    current = "".to_owned();
                }
            }
            '[' => {
                in_square = true;
                if !current.is_empty() {
                    vec.push(json!(current));
                    current = "".to_owned();
                }
            }
            ']' => {
                in_square = false;
                if prev == "]" {
                    break;
                }
                let len = current.len();
                if len >= 2 {
                    let mut cs = current.chars();
                    let first = cs.next().unwrap();
                    let last = cs.last().unwrap();
                    if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
                        current = current[1..(len - 1)].to_owned();
                    }
                }
                vec.push(json!(current));
                current = "".to_owned();
            }
            _ => {
                current.push_str(&c.to_string());
            }
        }
        prev = c.to_string();
    }
    if prev == "." || !current.is_empty() {
        vec.push(json!(current));
    }
    Value::Array(vec)
}
///
pub fn to_path_x(value: Value) -> Vec<String> {
    match value {
        Value::Null => vec![],
        Value::Bool(_) | Value::Number(_) | Value::Object(_) => vec![to_string_x(value)],
        Value::String(s) => x_to_path_x(&s),
        Value::Array(vec) => vec.into_iter().map(to_string_x).collect::<Vec<String>>(),
    }
}
/// See lodash [toPath](https://lodash.com/docs/#toPath)
pub fn to_path(value: Value) -> Value {
    match value {
        Value::Null => json!([]),
        Value::Bool(_) | Value::Number(_) | Value::Object(_) => json!([to_string_x(value)]),
        Value::String(s) => x_to_path(&s),
        Value::Array(vec) => {
            json!(vec.into_iter().map(to_string).collect::<Vec<Value>>())
        }
    }
}

/// Based on [x_to_path_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   x_to_path_x!("a.b.c"),
///   vec!["a", "b", "c"]
/// );
/// assert_eq!(
///   x_to_path_x!("a[0].b.c"),
///   vec!["a", "0", "b", "c"]
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// ```
#[macro_export]
macro_rules! x_to_path_x {
    () => {
        vec![]
    };
    ($a:expr $(,)*) => {
        $crate::x_to_path_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_to_path_x($a)
    };
}
/// Based on [x_to_path()]
#[macro_export]
macro_rules! x_to_path {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::x_to_path($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_to_path($a)
    };
}
/// Based on [to_path_x()]
#[macro_export]
macro_rules! to_path_x {
    () => {
        vec![]
    };
    ($a:expr $(,)*) => {
        $crate::to_path_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_path_x($a)
    };
}
/// Based on [to_path()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   to_path!(json!("a.b.c")),
///   json!(['a', 'b', 'c'])
/// );
/// assert_eq!(
///   to_path!(json!("a[0].b.c")),
///   json!(['a', '0', 'b', 'c'])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_path!(), json!([]));
/// assert_eq!(to_path!(json!(null)), json!([]));
/// assert_eq!(to_path!(json!(false)), json!(["false"]));
/// assert_eq!(to_path!(json!(0)), json!(["0"]));
/// assert_eq!(to_path!(json!("")), json!([]));
/// assert_eq!(to_path!(json!("..")), json!(["","",""]));
/// assert_eq!(to_path!(json!(r#"[""]"#)), json!([""]));
/// assert_eq!(to_path!(json!(r#".a[b]"#)), json!(["","a","b"]));
/// assert_eq!(to_path!(json!(r#"a['b']"#)), json!(["a","b"]));
/// assert_eq!(to_path!(json!(r#"a["b"]"#)), json!(["a","b"]));
/// assert_eq!(to_path!(json!(r#"a['"b"']"#)), json!([ "a", r#""b""# ]));
/// assert_eq!(to_path!(json!(r#"a["'b'"]"#)), json!([ "a", r#"'b'"# ]));
/// assert_eq!(to_path!(json!(r#"a[[b]c]"#)), json!(["a","b","c"]));
/// assert_eq!(to_path!(json!(r#"a[b[c]]"#)), json!(["a","b","c"]));
/// assert_eq!(to_path!(json!(r#"a[b"#)), json!(["a","b"]));
/// assert_eq!(to_path!(json!(r#"a[[b"#)), json!(["a","b"]));
/// assert_eq!(to_path!(json!(r#"a['b"#)), json!(["a",r#"'b"#]));
/// assert_eq!(to_path!(json!(r#"a["b"#)), json!(["a",r#""b"#]));
/// assert_eq!(to_path!(json!([])), json!([]));
/// assert_eq!(to_path!(json!(["x", 0, "y", "z"])), json!(["x", "0", "y", "z"]));
/// assert_eq!(to_path!(json!({})), json!(["serde_json::map::Map<alloc::string::String, serde_json::value::Value>"]));
/// assert_eq!(to_path!(json!({"a":1})), json!(["serde_json::map::Map<alloc::string::String, serde_json::value::Value>"]));
/// ```
#[macro_export]
macro_rules! to_path {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_path($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_path($a)
    };
}

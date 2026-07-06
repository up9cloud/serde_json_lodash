use crate::lib::{Value};

/// See lodash [difference](https://lodash.com/docs/#difference)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::difference;
/// # use serde_json::json;
/// assert_eq!(difference(json!([2,1]), json!([2,3])), json!([1]));
/// ```
pub fn difference(v1: Value, v2: Value) -> Value {
    let mut result = vec![];
    let v1 = match v1 {
        Value::Null => return Value::Array(result),
        Value::Bool(_) => return Value::Array(result),
        Value::Number(_) => return Value::Array(result),
        Value::String(_) => return Value::Array(result),
        Value::Array(vec) => vec,
        Value::Object(_) => return Value::Array(result),
    };
    match v2 {
        Value::Null => return Value::Array(v1),
        Value::Bool(_) => return Value::Array(v1),
        Value::Number(_) => return Value::Array(v1),
        Value::String(_) => return Value::Array(v1),
        Value::Array(vec) => {
            'outer: for v in v1.iter() {
                match v {
                    Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
                        for vv in vec.iter() {
                            match vv {
                                Value::Null => {
                                    if v.is_null() {
                                        continue 'outer;
                                    }
                                }
                                Value::Bool(_) | Value::Number(_) | Value::String(_) => {
                                    if v == vv {
                                        continue 'outer;
                                    }
                                }
                                Value::Array(_) => continue,
                                Value::Object(_) => continue,
                            }
                        }
                    }
                    Value::Array(_) => (),
                    Value::Object(_) => (),
                }
                result.push(v.clone())
            }
        }
        Value::Object(_) => return Value::Array(v1),
    };
    Value::Array(result)
}

/// Based on [difference()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   difference!(json!([2,1]), json!([2,3])),
///   json!([1])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(difference!(), json!([]));
/// assert_eq!(difference!(json!(null)), json!([]));
/// assert_eq!(difference!(json!(true)), json!([]));
/// assert_eq!(difference!(json!(0)), json!([]));
/// assert_eq!(difference!(json!("")), json!([]));
/// assert_eq!(difference!(json!([1,null])), json!([1,null]));
/// assert_eq!(difference!(json!({})), json!([]));
/// assert_eq!(difference!(json!([null,true,0,"",1.1,[],{}]), json!([null,true,0,"",1.1,[],{}])), json!([ [], {} ]));
/// ```
#[macro_export]
macro_rules! difference {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::difference($a, $crate::lib::json!([]))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::difference($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::difference($a, $b)
    };
}

/// `_x` helper for [difference()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [difference()] and read the returned `Value`.
pub fn difference_x() {
    todo!()
}

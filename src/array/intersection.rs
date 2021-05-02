use crate::lib::{Value};

#[doc(hidden)]
pub fn _empty_array() -> Vec<Value> {
    vec![]
}
///
pub fn intersection_x(v1: Value, v2: Value) -> Vec<Value> {
    let mut result = vec![];
    let vec1 = match v1 {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return result
        }
        Value::Array(vec) => {
            if vec.is_empty() {
                return result;
            }
            vec
        }
    };
    match v2 {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return result
        }
        Value::Array(vec2) => {
            if vec2.is_empty() {
                return result;
            }
            for v1 in vec1.iter() {
                // Based on the RFC https://262.ecma-international.org/7.0/#sec-samevaluenonnumber
                // -- if they are same Object value return true
                // For here we compare two Value, not &Value. It means v1 and v2 are absolutely not same object in memory.
                // So we could just simply ignore them
                if v1.is_object() || v1.is_array() {
                    continue;
                }
                for v2 in vec2.iter() {
                    if v2.is_object() || v2.is_array() {
                        continue;
                    }
                    if v1 == v2 {
                        result.push(v1.clone())
                    }
                }
            }
        }
    }
    result
}
/// See lodash [intersection](https://lodash.com/docs/#intersection)
pub fn intersection(v1: Value, v2: Value) -> Value {
    Value::Array(intersection_x(v1, v2))
}

/// Based on [intersection_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   intersection_x!(json!([2, 1]), json!([2, 3])),
///   vec![json!(2)]
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::{json, Value};
/// let expect: Vec<Value> = vec![];
/// assert_eq!(intersection_x!(), expect);
/// assert_eq!(intersection_x!(json!([3, 2, 1]), json!([2, 3, 4]), json!([3, 2, 0])), vec![json!(3), json!(2)])
/// ```
#[macro_export]
macro_rules! intersection_x {
    () => (
        $crate::_empty_array()
    );
    ($a:expr $(,)*) => {{
        if $a.is_array() {
            $a.as_array().unwrap_or_else($crate::_empty_array)
        } else {
            $crate::_empty_array()
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::intersection_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::intersection_x!($crate::intersection($a, $b), $($rest)*)
    };
}
/// Based on [intersection()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   intersection!(json!([2, 1]), json!([2, 3])),
///   json!([2])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(intersection!(), json!([]));
/// assert_eq!(intersection!(json!(null)), json!([]));
/// assert_eq!(intersection!(json!(false)), json!([]));
/// assert_eq!(intersection!(json!(0)), json!([]));
/// assert_eq!(intersection!(json!("")), json!([]));
/// assert_eq!(intersection!(json!("ab")), json!([]));
/// assert_eq!(intersection!(json!([])), json!([]));
/// assert_eq!(intersection!(json!({})), json!([]));
/// assert_eq!(intersection!(json!([null,false,0,"","ab",[],{}])), json!([null,false,0,"","ab",[],{}]));
/// assert_eq!(intersection!(json!([null,false,0,"","ab",[],{}]), json!([])), json!([]));
/// assert_eq!(intersection!(json!([null,false,0,"","ab",[],{}]), json!([null,false,0,"","ab",[],{}])), json!([null,false,0,"","ab"]));
/// assert_eq!(intersection!(json!([null, false, 1]), json!([null,false,0]), json!([false, 2, null])), json!([null,false]));
/// ```
#[macro_export]
macro_rules! intersection {
    () => (
        json!([])
    );
    ($a:expr $(,)*) => {{
        if $a.is_array() {
            $a
        } else {
            json!([])
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::intersection($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::intersection!($crate::intersection($a, $b), $($rest)*)
    };
}

use crate::lib::{json, Value};

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
///
pub fn intersection(v1: Value, v2: Value) -> Value {
    Value::Array(intersection_x(v1, v2))
}

#[doc(hidden)]
pub fn _return_self_if_array(v: Value) -> Value {
    if v.is_array() {
        return v;
    }
    json!([])
}

/// Description can be found in [lodash intersection](https://lodash.com/docs/#intersection)
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
/// ```
#[macro_export]
macro_rules! intersection {
    () => (
        json!([])
    );
    ($a:expr $(,)*) => {
        $crate::_return_self_if_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::intersection($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::intersection!($crate::intersection($a, $b), $($rest)*)
    };
}

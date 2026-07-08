use crate::internal::SvzRef;
use crate::lib::Value;

use std::collections::HashSet;

/// Fn form of [difference!](crate::difference!); see it for the full docs
///
/// `_x` form: **not provided** — see [difference_x()]
///
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
            // Only scalars can match (SameValueZero: two owned composites are
            // never the same reference), so hash vec2's scalars once — O(n+m).
            let set2: HashSet<SvzRef> = vec
                .iter()
                .filter(|v| !v.is_object() && !v.is_array())
                .map(SvzRef)
                .collect();
            for v in v1.iter() {
                match v {
                    Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
                        if set2.contains(&SvzRef(v)) {
                            continue;
                        }
                    }
                    Value::Array(_) | Value::Object(_) => (),
                }
                result.push(v.clone())
            }
        }
        Value::Object(_) => return Value::Array(v1),
    };
    Value::Array(result)
}

/// See lodash [difference](https://lodash.com/docs/#difference)
///
/// Fn form: [difference()] | `_x` form: **not provided** — see [difference_x()]
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
/// // SameValueZero: JS has one number type, so 1 == 1.0
/// assert_eq!(difference!(json!([1, 2]), json!([1.0])), json!([2]));
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

build_not_provided_x!(difference, difference_x);

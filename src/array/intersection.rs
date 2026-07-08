use crate::internal::SvzRef;
use crate::lib::Value;

use std::collections::HashSet;

#[doc(hidden)]
pub fn _empty_array() -> Vec<Value> {
    vec![]
}

/// Fn form of [intersection!](crate::intersection!); see it for the full docs
///
/// `_x` forms: [intersection_x!](crate::intersection_x!), [intersection_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::intersection;
/// # use serde_json::json;
/// assert_eq!(intersection(json!([2, 1]), json!([2, 3])), json!([2]));
/// ```
pub fn intersection(v1: Value, v2: Value) -> Value {
    Value::Array(intersection_x(v1, v2))
}

/// See lodash [intersection](https://lodash.com/docs/#intersection)
///
/// Fn form: [intersection()] | `_x` forms: [intersection_x!](crate::intersection_x!), [intersection_x()]
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
/// Additional cases:
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
/// // duplicates in the inputs still yield unique values, like lodash
/// assert_eq!(intersection!(json!([2, 1, 2]), json!([2, 3, 2])), json!([2]));
/// // SameValueZero: JS has one number type, so 1 == 1.0
/// assert_eq!(intersection!(json!([1, 3]), json!([1.0])), json!([1]));
/// ```
#[macro_export]
macro_rules! intersection {
    () => (
        $crate::lib::json!([])
    );
    ($a:expr $(,)*) => {{
        if $a.is_array() {
            $a
        } else {
            $crate::lib::json!([])
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::intersection($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::intersection!($crate::intersection($a, $b), $($rest)*)
    };
}

/// `_x` helper for [intersection!](crate::intersection!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [intersection_x!](crate::intersection_x!) | `Value` forms: [intersection!](crate::intersection!), [intersection()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::intersection_x;
/// # use serde_json::json;
/// assert_eq!(intersection_x(json!([2, 1]), json!([2, 3])), vec![json!(2)]);
/// ```
pub fn intersection_x(v1: Value, v2: Value) -> Vec<Value> {
    let (vec1, vec2) = match (v1, v2) {
        (Value::Array(a), Value::Array(b)) if !a.is_empty() && !b.is_empty() => (a, b),
        _ => return vec![],
    };
    // Objects/arrays are skipped entirely: SameValueZero
    // (https://262.ecma-international.org/7.0/#sec-samevaluenonnumber) matches
    // them by reference identity, and two owned `Value`s are never the same
    // object. Scalars intersect by value through a hash set (`Value: Hash`
    // is consistent with its `Eq`), unique and in first-array order, in
    // O(len1 + len2).
    let set2: HashSet<SvzRef> = vec2
        .iter()
        .filter(|v| !v.is_object() && !v.is_array())
        .map(SvzRef)
        .collect();
    let mut emitted: HashSet<SvzRef> = HashSet::with_capacity(set2.len());
    let mut result = vec![];
    for v in vec1.iter() {
        if v.is_object() || v.is_array() {
            continue;
        }
        if set2.contains(&SvzRef(v)) && emitted.insert(SvzRef(v)) {
            result.push(v.clone());
        }
    }
    result
}

/// `_x` helper for [intersection!](crate::intersection!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [intersection_x()] | `Value` forms: [intersection!](crate::intersection!), [intersection()]
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
/// Additional cases:
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

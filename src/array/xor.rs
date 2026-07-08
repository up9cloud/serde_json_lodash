use crate::internal::SvzRef;
use crate::lib::Value;

use std::collections::HashSet;

/// Fn form of [xor!](crate::xor!); see it for the full docs
///
/// `_x` form: **not provided** — see [xor_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::xor;
/// # use serde_json::json;
/// assert_eq!(xor(json!([2, 1]), json!([2, 3])), json!([1, 3]));
/// ```
pub fn xor(array: Value, other: Value) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => vec![],
    };
    let b = match other {
        Value::Array(v) => v,
        _ => vec![],
    };
    let a_set: HashSet<SvzRef> = a.iter().map(SvzRef).collect();
    let b_set: HashSet<SvzRef> = b.iter().map(SvzRef).collect();
    let mut seen: HashSet<SvzRef> = HashSet::new();
    let mut out = vec![];
    for v in a.iter() {
        if !b_set.contains(&SvzRef(v)) && seen.insert(SvzRef(v)) {
            out.push(v.clone());
        }
    }
    for v in b.iter() {
        if !a_set.contains(&SvzRef(v)) && seen.insert(SvzRef(v)) {
            out.push(v.clone());
        }
    }
    Value::Array(out)
}

/// See lodash [xor](https://lodash.com/docs/#xor)
///
/// Returns the symmetric difference: values present in exactly one of the
/// two arrays
///
/// Fn form: [xor()] | `_x` form: **not provided** — see [xor_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(xor!(json!([2, 1]), json!([2, 3])), json!([1, 3]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(xor!(), json!([]));
/// assert_eq!(xor!(json!([1, 2])), json!([1, 2]));
/// assert_eq!(xor!(json!([1, 2]), json!([1, 2])), json!([]));
/// // SameValueZero: JS has one number type, so 1 == 1.0
/// assert_eq!(xor!(json!([1, 2]), json!([1.0])), json!([2]));
/// ```
#[macro_export]
macro_rules! xor {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $($b:expr),+ $(,)*) => {{
        let mut acc = $crate::uniq($a);
        $(
            acc = $crate::xor(acc, $b);
        )+
        acc
    }};
}

build_not_provided_x!(xor, xor_x);

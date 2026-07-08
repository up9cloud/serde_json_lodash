use crate::internal::Svz;
use crate::lib::{Value, json};

use std::collections::HashSet;

/// Fn form of [without!](crate::without!); see it for the full docs
///
/// `_x` form: **not provided** — see [without_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::without;
/// # use serde_json::json;
/// assert_eq!(without(json!([2, 1, 2, 3]), json!([1, 2])), json!([3]));
/// ```
pub fn without(array: Value, values: Value) -> Value {
    match array {
        Value::Array(vec) => {
            let excluded: HashSet<Svz> = match values {
                Value::Array(v) => v.into_iter().map(Svz).collect(),
                _ => HashSet::new(),
            };
            Value::Array(
                vec.into_iter()
                    .map(Svz)
                    .filter(|k| !excluded.contains(k))
                    .map(|k| k.0)
                    .collect(),
            )
        }
        _ => json!([]),
    }
}

/// See lodash [without](https://lodash.com/docs/#without)
///
/// `values` is an array of elements to exclude
///
/// Fn form: [without()] | `_x` form: **not provided** — see [without_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   without!(json!([2, 1, 2, 3]), json!([1, 2])),
///   json!([3])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(without!(), json!([]));
/// assert_eq!(without!(json!([1, 2, 3])), json!([1, 2, 3]));
/// assert_eq!(without!(json!([1, 2, 3]), json!([9])), json!([1, 2, 3]));
/// // SameValueZero: JS has one number type, so 1 == 1.0
/// assert_eq!(without!(json!([1, 1.0, 2]), json!([1])), json!([2]));
/// ```
#[macro_export]
macro_rules! without {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::without($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::without($a, $b)
    };
}

build_not_provided_x!(without, without_x);

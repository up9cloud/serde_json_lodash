use crate::lib::Value;

use crate::internal::uniq_by_key;

/// Fn form of [union!](crate::union!); see it for the full docs
///
/// `_x` form: **not provided** — see [union_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::union;
/// # use serde_json::json;
/// assert_eq!(union(json!([2]), json!([1, 2])), json!([2, 1]));
/// ```
pub fn union(array: Value, other: Value) -> Value {
    let mut all = vec![];
    if let Value::Array(vec) = array {
        all.extend(vec);
    }
    if let Value::Array(vec) = other {
        all.extend(vec);
    }
    Value::Array(uniq_by_key(all, |v| v.clone()))
}

/// See lodash [union](https://lodash.com/docs/#union)
///
/// Fn form: [union()] | `_x` form: **not provided** — see [union_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(union!(json!([2]), json!([1, 2])), json!([2, 1]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(union!(), json!([]));
/// assert_eq!(union!(json!([1, 2])), json!([1, 2]));
/// assert_eq!(union!(json!([1]), json!([2]), json!([1, 3])), json!([1, 2, 3]));
/// // SameValueZero: JS has one number type, so 1 == 1.0
/// assert_eq!(union!(json!([1]), json!([1.0, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! union {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $($b:expr),+ $(,)*) => {{
        let mut acc = $crate::uniq($a);
        $(
            acc = $crate::union(acc, $b);
        )+
        acc
    }};
}

build_not_provided_x!(union, union_x);

use crate::lib::Value;

/// See lodash [unionWith](https://lodash.com/docs/#unionWith)
///
/// `comparator` is invoked to compare elements for uniqueness
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::union_with;
/// # use serde_json::json;
/// assert_eq!(union_with(json!([1, 2]), json!([2, 3]), |a, b| a == b), json!([1, 2, 3]));
/// ```
pub fn union_with(array: Value, other: Value, comparator: fn(&Value, &Value) -> bool) -> Value {
    let mut all = vec![];
    if let Value::Array(vec) = array {
        all.extend(vec);
    }
    if let Value::Array(vec) = other {
        all.extend(vec);
    }
    let mut out: Vec<Value> = vec![];
    for v in all {
        if !out.iter().any(|kept| comparator(kept, &v)) {
            out.push(v);
        }
    }
    Value::Array(out)
}

/// Based on [union_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   union_with!(json!([1, 2]), json!([2, 3]), |a, b| a == b),
///   json!([1, 2, 3])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(union_with!(), json!([]));
/// assert_eq!(union_with!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! union_with {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::concat($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::union_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::union_with($a, $b, $c)
    };
}

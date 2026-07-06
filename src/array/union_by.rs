use crate::lib::Value;
use crate::internal::uniq_by_key;

/// See lodash [unionBy](https://lodash.com/docs/#unionBy)
///
/// `iteratee` maps each element to the value used for uniqueness
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::union_by;
/// # use serde_json::json;
/// assert_eq!(union_by(json!([2.1]), json!([1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())), json!([2.1, 1.2]));
/// ```
pub fn union_by(array: Value, other: Value, iteratee: fn(&Value) -> Value) -> Value {
    let mut all = vec![];
    if let Value::Array(vec) = array {
        all.extend(vec);
    }
    if let Value::Array(vec) = other {
        all.extend(vec);
    }
    Value::Array(uniq_by_key(all, iteratee))
}

/// Based on [union_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   union_by!(json!([2.1]), json!([1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([2.1, 1.2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(union_by!(), json!([]));
/// assert_eq!(union_by!(json!([1, 1])), json!([1]));
/// ```
#[macro_export]
macro_rules! union_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::union($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::union_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::union_by($a, $b, $c)
    };
}

/// `_x` helper for [union_by()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [union_by()] and read the returned `Value`.
pub fn union_by_x() {
    todo!()
}

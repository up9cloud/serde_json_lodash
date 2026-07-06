use crate::lib::Value;
use crate::internal::uniq_by_key;

/// See lodash [union](https://lodash.com/docs/#union)
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

/// Based on [union()]
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

/// `_x` helper for [union()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [union()] and read the returned `Value`.
pub fn union_x() {
    todo!()
}
/// Based on [union_x()]
#[macro_export]
macro_rules! union_x {
    ($($t:tt)*) => {
        $crate::union_x()
    };
}

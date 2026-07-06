use crate::lib::Value;

/// See lodash [forOwnRight](https://lodash.com/docs/#forOwnRight)
///
/// Like [for_own()](fn@crate::for_own) but iterates in reverse key order
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::for_own_right;
/// # use serde_json::json;
/// assert_eq!(for_own_right(json!({"a": 1, "b": 2}), |_v, _k| true), json!({"a": 1, "b": 2}));
/// ```
pub fn for_own_right(object: Value, iteratee: fn(&Value, &str) -> bool) -> Value {
    if let Value::Object(o) = &object {
        for (k, v) in o.iter().rev() {
            if !iteratee(v, k) {
                break;
            }
        }
    }
    object
}

/// Based on [for_own_right()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// for_own_right!(json!({ "a": 1, "b": 2 }), |_v, k| { println!("{}", k); true });
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(for_own_right!(), json!(null));
/// assert_eq!(for_own_right!(json!({"a": 1})), json!({"a": 1}));
/// assert_eq!(for_own_right!(json!({"a": 1, "b": 2}), |_v, _k| true), json!({"a": 1, "b": 2}));
/// ```
#[macro_export]
macro_rules! for_own_right {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::for_own_right($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::for_own_right($a, $b)
    };
}

use crate::lib::Value;

/// See lodash [forOwn](https://lodash.com/docs/#forOwn)
///
/// Iterates over own properties invoking `iteratee(value, key)`. Returning
/// `false` from `iteratee` stops iteration early. Returns `object`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::for_own;
/// # use serde_json::json;
/// assert_eq!(for_own(json!({"a": 1, "b": 2}), |_v, _k| true), json!({"a": 1, "b": 2}));
/// ```
pub fn for_own(object: Value, iteratee: fn(&Value, &str) -> bool) -> Value {
    if let Value::Object(o) = &object {
        for (k, v) in o {
            if !iteratee(v, k) {
                break;
            }
        }
    }
    object
}

/// Based on [for_own()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // iteratee returns true to keep going
/// let object = json!({ "a": 1, "b": 2 });
/// assert_eq!(for_own!(object.clone(), |_v, k| { println!("{}", k); true }), object);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(for_own!(), json!(null));
/// assert_eq!(for_own!(json!({"a": 1})), json!({"a": 1}));
/// assert_eq!(for_own!(json!({"a": 1, "b": 2}), |_v, _k| true), json!({"a": 1, "b": 2}));
/// ```
#[macro_export]
macro_rules! for_own {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::for_own($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::for_own($a, $b)
    };
}

/// `_x` helper for [for_own()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [for_own()] and read the returned `Value`.
pub fn for_own_x() {
    todo!()
}

use crate::lib::Value;

/// Fn form of [for_own!](crate::for_own!); see it for the full docs
///
/// `_x` form: **not provided** — see [for_own_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::for_own;
/// # use serde_json::json;
/// assert_eq!(for_own(json!({"a": 1, "b": 2}), |_v, _k| true), json!({"a": 1, "b": 2}));
/// ```
pub fn for_own(object: Value, iteratee: impl Fn(&Value, &str) -> bool) -> Value {
    if let Value::Object(o) = &object {
        for (k, v) in o {
            if !iteratee(v, k) {
                break;
            }
        }
    }
    object
}

/// See lodash [forOwn](https://lodash.com/docs/#forOwn)
///
/// Iterates over own properties invoking `iteratee(value, key)`. Returning
/// `false` from `iteratee` stops iteration early. Returns `object`
///
/// Fn form: [for_own()] | `_x` form: **not provided** — see [for_own_x()]
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

build_not_provided_x!(for_own, for_own_x);

use crate::lib::Value;

/// Fn form of [for_own_right!](crate::for_own_right!); see it for the full docs
///
/// `_x` form: **not provided** — see [for_own_right_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::for_own_right;
/// # use serde_json::json;
/// assert_eq!(for_own_right(json!({"a": 1, "b": 2}), |_v, _k| true), json!({"a": 1, "b": 2}));
/// ```
pub fn for_own_right(object: Value, iteratee: impl Fn(&Value, &str) -> bool) -> Value {
    if let Value::Object(o) = &object {
        for (k, v) in o.iter().rev() {
            if !iteratee(v, k) {
                break;
            }
        }
    }
    object
}

/// See lodash [forOwnRight](https://lodash.com/docs/#forOwnRight)
///
/// Like [for_own()](fn@crate::for_own) but iterates in reverse key order
///
/// Fn form: [for_own_right()] | `_x` form: **not provided** — see [for_own_right_x()]
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [for_own_right!](crate::for_own_right!) and read the
/// returned `Value`.
///
/// Macro form: [for_own_right_x!](crate::for_own_right_x!)
pub fn for_own_right_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [for_own_right!](crate::for_own_right!) and read the
/// returned `Value`.
///
/// Fn form: [for_own_right_x()]
#[macro_export]
macro_rules! for_own_right_x {
    ($($t:tt)*) => {
        $crate::for_own_right_x()
    };
}

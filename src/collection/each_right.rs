use crate::lib::Value;

/// Fn form of [each_right!](crate::each_right!); see it for the full docs
///
/// `_x` form: **not provided** — see [each_right_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::each_right;
/// # use serde_json::json;
/// assert_eq!(each_right(json!([1, 2, 3]), |_| true), json!([1, 2, 3]));
/// ```
pub fn each_right(collection: Value, iteratee: impl Fn(&Value) -> bool) -> Value {
    // borrow-iterate in reverse; see [each()] for why nothing is cloned
    match &collection {
        Value::Array(vec) => {
            for v in vec.iter().rev() {
                if !iteratee(v) {
                    break;
                }
            }
        }
        Value::Object(o) => {
            for v in o.values().rev() {
                if !iteratee(v) {
                    break;
                }
            }
        }
        Value::String(s) => {
            for c in s.chars().rev() {
                if !iteratee(&Value::String(c.to_string())) {
                    break;
                }
            }
        }
        _ => {}
    }
    collection
}

/// See lodash [forEachRight](https://lodash.com/docs/#forEachRight)
///
/// Like [each()](fn@crate::each) but iterates from the end
///
/// Fn form: [each_right()] | `_x` form: **not provided** — see [each_right_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([1, 2]);
/// assert_eq!(each_right!(array.clone(), |n| { println!("{}", n); true }), array);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(each_right!(), json!(null));
/// assert_eq!(each_right!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! each_right {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::each_right($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::each_right($a, $b)
    };
}

build_not_provided_x!(each_right, each_right_x);

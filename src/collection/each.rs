use crate::lib::Value;

/// Fn form of [each!](crate::each!); see it for the full docs
///
/// `_x` form: **not provided** — see [each_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::each;
/// # use serde_json::json;
/// assert_eq!(each(json!([1, 2, 3]), |_| true), json!([1, 2, 3]));
/// ```
pub fn each(collection: Value, iteratee: impl Fn(&Value) -> bool) -> Value {
    // borrow-iterate: `collection` is returned, so its values must not be
    // moved out, and cloning them just to visit would be a waste
    match &collection {
        Value::Array(vec) => {
            for v in vec {
                if !iteratee(v) {
                    break;
                }
            }
        }
        Value::Object(o) => {
            for v in o.values() {
                if !iteratee(v) {
                    break;
                }
            }
        }
        Value::String(s) => {
            for c in s.chars() {
                if !iteratee(&Value::String(c.to_string())) {
                    break;
                }
            }
        }
        _ => {}
    }
    collection
}

/// See lodash [forEach](https://lodash.com/docs/#forEach)
///
/// Invokes `iteratee` for each element; returning `false` stops iteration.
/// Returns `collection`
///
/// Fn form: [each()] | `_x` form: **not provided** — see [each_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([1, 2]);
/// assert_eq!(each!(array.clone(), |n| { println!("{}", n); true }), array);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(each!(), json!(null));
/// assert_eq!(each!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! each {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::each($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::each($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [each!](crate::each!) and read the returned `Value`.
///
/// Macro form: [each_x!](crate::each_x!)
pub fn each_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [each!](crate::each!) and read the returned `Value`.
///
/// Fn form: [each_x()]
#[macro_export]
macro_rules! each_x {
    ($($t:tt)*) => {
        $crate::each_x()
    };
}

use crate::internal::same_value_zero;
use crate::lib::Value;

/// Fn form of [pull!](crate::pull!); see it for the full docs
///
/// `_x` form: **not provided** — see [pull_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pull;
/// # use serde_json::json;
/// assert_eq!(pull(json!([[]]), json!([])), json!([[]]));
/// ```
pub fn pull(mut array: Value, value: Value) -> Value {
    let new_vec = match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return array;
        }
        Value::Array(ref vec) => {
            match value {
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => (),
                Value::Array(_) | Value::Object(_) => return array,
            }
            let mut new_vec = vec![];
            for item in vec.iter() {
                if !same_value_zero(item, &value) {
                    new_vec.push(item.clone())
                }
            }
            new_vec
        }
    };
    *array.as_array_mut().unwrap() = new_vec;
    array
}

/// See lodash [pull](https://lodash.com/docs/#pull)
///
/// Fn form: [pull()] | `_x` form: **not provided** — see [pull_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let mut array = json!(['a', 'b', 'c', 'a', 'b', 'c']);
/// array = pull!(array, json!('a'), json!('c'));
/// assert_eq!(
///   array,
///   json!(['b', 'b'])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pull!(), json!(null));
/// assert_eq!(pull!(json!(null)), json!(null));
/// assert_eq!(pull!(json!(false)), json!(false));
/// assert_eq!(pull!(json!(0)), json!(0));
/// assert_eq!(pull!(json!("")), json!(""));
/// assert_eq!(pull!(json!([])), json!([]));
/// assert_eq!(pull!(json!([[]]), json!([])), json!([[]]));
/// assert_eq!(pull!(json!([{}]), json!({})), json!([{}]));
/// assert_eq!(pull!(json!({})), json!({}));
/// // SameValueZero: JS has one number type, so 1 == 1.0
/// assert_eq!(pull!(json!([1, 1.0, 2]), json!(1)), json!([2]));
/// ```
#[macro_export]
macro_rules! pull {
    () => (
        $crate::lib::json!(null)
    );
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pull($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::pull!($crate::pull($a, $b), $($rest)*)
    };
}

build_not_provided_x!(pull, pull_x);

use crate::lib::{Value};

/// See lodash [pullAll](https://lodash.com/docs/#pullAll)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pull_all;
/// # use serde_json::json;
/// assert_eq!(pull_all(json!([1, 2, 3]), json!([2])), json!([1, 3]));
/// ```
pub fn pull_all(mut array: Value, values: Value) -> Value {
    let new_vec = match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return array;
        }
        Value::Array(ref vec) => {
            let values_vec = match values {
                Value::Null
                | Value::Bool(_)
                | Value::Number(_)
                | Value::String(_)
                | Value::Object(_) => return array,
                Value::Array(vec) => vec,
            };
            let mut new_vec = vec![];
            'a: for item in vec.iter() {
                for value in values_vec.iter() {
                    if item == value {
                        continue 'a;
                    }
                }
                new_vec.push(item.clone())
            }
            new_vec
        }
    };
    *array.as_array_mut().unwrap() = new_vec;
    array
}

/// Based on [pull_all()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
// => ['b', 'b']
/// let mut array = json!(['a', 'b', 'c', 'a', 'b', 'c']);
/// array = pull_all!(array, json!(['a', 'c']));
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
/// assert_eq!(pull_all!(), json!(null));
/// assert_eq!(pull_all!(json!(null)), json!(null));
/// assert_eq!(pull_all!(json!(false)), json!(false));
/// assert_eq!(pull_all!(json!(0)), json!(0));
/// assert_eq!(pull_all!(json!("")), json!(""));
/// assert_eq!(pull_all!(json!([])), json!([]));
/// assert_eq!(pull_all!(json!([[]]), json!([])), json!([[]]));
/// assert_eq!(pull_all!(json!([{}]), json!({})), json!([{}]));
/// assert_eq!(pull_all!(json!([null]), json!([null])), json!([]));
/// assert_eq!(pull_all!(json!([null,0]), json!([null]), json!([0])), json!([0]));
/// assert_eq!(pull_all!(json!({})), json!({}));
/// ```
#[macro_export]
macro_rules! pull_all {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pull_all($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::pull_all($a, $b)
    };
}

/// `_x` helper for [pull_all()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [pull_all()] and read the returned `Value`.
pub fn pull_all_x() {
    todo!()
}

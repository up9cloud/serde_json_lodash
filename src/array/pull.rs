use crate::lib::{Value};

/// See lodash [pull](https://lodash.com/docs/#pull)
pub fn pull(mut array: Value, value: Value) -> Value {
    let new_vec = match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return array
        }
        Value::Array(ref vec) => {
            match value {
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => (),
                Value::Array(_) | Value::Object(_) => return array,
            }
            let mut new_vec = vec![];
            for item in vec.iter() {
                if item != &value {
                    new_vec.push(item.clone())
                }
            }
            new_vec
        }
    };
    *array.as_array_mut().unwrap() = new_vec;
    array
}

/// Based on [pull()]
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
/// More examples:
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
/// ```
#[macro_export]
macro_rules! pull {
    () => (
        json!(null)
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

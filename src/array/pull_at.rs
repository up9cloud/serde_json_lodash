use crate::lib::{Value};

/// See lodash [pullAt](https://lodash.com/docs/#pullAt)
pub fn pull_at(array: &mut Value, indexes: Vec<usize>) -> Value {
    let (new_vec, pulled_vec) = match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            (vec![], vec![Value::Null; indexes.len()])
        }
        Value::Array(ref vec) => {
            let mut pulled_vec = vec![];
            let mut new_vec = vec![];
            let mut uniq_vec = vec![false; vec.len()];
            for i in indexes.into_iter() {
                if i > (vec.len() - 1) {
                    pulled_vec.push(Value::Null);
                } else {
                    pulled_vec.push(vec[i].clone());
                    uniq_vec[i] = true
                }
            }
            for (i, pulled) in uniq_vec.into_iter().enumerate() {
                if !pulled {
                    new_vec.push(vec[i].clone());
                }
            }
            (new_vec, pulled_vec)
        }
    };
    *array.as_array_mut().unwrap() = new_vec;
    Value::Array(pulled_vec)
}

/// Based on [pull_at()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let mut array = json!(['a', 'b', 'c', 'd']);
/// let pulled = pull_at!(&mut array, vec![1, 3]);
/// assert_eq!(
///   array,
///   json!(['a', 'c'])
/// );
/// assert_eq!(
///   pulled,
///   json!(['b', 'd'])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pull_at!(), json!([]));
/// assert_eq!(pull_at!(json!(null)), json!([]));
/// assert_eq!(pull_at!(json!(false)), json!([]));
/// assert_eq!(pull_at!(json!(0)), json!([]));
/// assert_eq!(pull_at!(json!("")), json!([]));
/// assert_eq!(pull_at!(json!([])), json!([]));
/// assert_eq!(pull_at!(json!({})), json!([]));
/// ```
#[macro_export]
macro_rules! pull_at {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pull_at($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::pull_at($a, $b) // TODO: support multiple parameters...
    };
}

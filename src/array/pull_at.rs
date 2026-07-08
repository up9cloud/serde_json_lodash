use crate::lib::Value;

/// Fn form of [pull_at!](crate::pull_at!); see it for the full docs
///
/// `_x` form: **not provided** — see [pull_at_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pull_at;
/// # use serde_json::json;
/// let mut a = json!([1, 2, 3]);
/// assert_eq!(pull_at(&mut a, vec![0]), json!([1]));
/// ```
pub fn pull_at(array: &mut Value, indexes: Vec<usize>) -> Value {
    let (new_vec, pulled_vec) = match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            (vec![], vec![Value::Null; indexes.len()])
        }
        Value::Array(vec) => {
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

/// See lodash [pullAt](https://lodash.com/docs/#pullAt)
///
/// Fn form: [pull_at()] | `_x` form: **not provided** — see [pull_at_x()]
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
/// Additional cases:
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
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pull_at($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::pull_at($a, $b) // TODO: support multiple parameters...
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [pull_at!](crate::pull_at!) and read the returned
/// `Value`.
///
/// Macro form: [pull_at_x!](crate::pull_at_x!)
pub fn pull_at_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [pull_at!](crate::pull_at!) and read the returned
/// `Value`.
///
/// Fn form: [pull_at_x()]
#[macro_export]
macro_rules! pull_at_x {
    ($($t:tt)*) => {
        $crate::pull_at_x()
    };
}

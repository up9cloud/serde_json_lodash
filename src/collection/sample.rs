use crate::lib::Value;
use crate::internal::rand_f64;
use crate::collection::collect::collection_values;

/// See lodash [sample](https://lodash.com/docs/#sample)
///
/// Returns a random element, or `Null` for an empty collection
pub fn sample(collection: Value) -> Value {
    let vec = collection_values(&collection);
    if vec.is_empty() {
        return Value::Null;
    }
    let i = (rand_f64() * vec.len() as f64) as usize;
    vec[i.min(vec.len() - 1)].clone()
}

/// Based on [sample()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let picked = sample!(json!([1, 2, 3, 4]));
/// assert!(json!([1, 2, 3, 4]).as_array().unwrap().contains(&picked));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sample!(), json!(null));
/// assert_eq!(sample!(json!([])), json!(null));
/// assert_eq!(sample!(json!([7])), json!(7));
/// ```
#[macro_export]
macro_rules! sample {
    () => {
        serde_json::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::sample($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::sample($a)
    };
}

use crate::lib::Value;

use crate::internal::rand_f64;

use crate::collection::collect::collection_values;

/// Fn form of [sample!](crate::sample!); see it for the full docs
///
/// `_x` form: **not provided** — see [sample_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sample;
/// # use serde_json::json;
/// assert_eq!(sample(json!([])), json!(null));
/// ```
pub fn sample(collection: Value) -> Value {
    let vec = collection_values(&collection);
    if vec.is_empty() {
        return Value::Null;
    }
    let i = (rand_f64() * vec.len() as f64) as usize;
    vec[i.min(vec.len() - 1)].clone()
}

/// See lodash [sample](https://lodash.com/docs/#sample)
///
/// Returns a random element, or `Null` for an empty collection
///
/// Fn form: [sample()] | `_x` form: **not provided** — see [sample_x()]
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
/// Additional cases:
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
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::sample($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::sample($a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [sample!](crate::sample!) and read the returned
/// `Value`.
///
/// Macro form: [sample_x!](crate::sample_x!)
pub fn sample_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [sample!](crate::sample!) and read the returned
/// `Value`.
///
/// Fn form: [sample_x()]
#[macro_export]
macro_rules! sample_x {
    ($($t:tt)*) => {
        $crate::sample_x()
    };
}

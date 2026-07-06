use crate::lib::Value;
use crate::collection::collect::collection_values;
use crate::collection::shuffle::shuffle_vec;

/// See lodash [sampleSize](https://lodash.com/docs/#sampleSize)
///
/// Returns up to `n` random elements
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sample_size;
/// # use serde_json::json;
/// assert_eq!(sample_size(json!([1, 2, 3]), 0), json!([]));
/// ```
pub fn sample_size(collection: Value, n: usize) -> Value {
    let mut vec = shuffle_vec(collection_values(&collection));
    vec.truncate(n);
    Value::Array(vec)
}

/// Based on [sample_size()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let s = sample_size!(json!([1, 2, 3]), 2);
/// assert_eq!(s.as_array().unwrap().len(), 2);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sample_size!(), json!([]));
/// assert_eq!(sample_size!(json!([1, 2, 3]), 5).as_array().unwrap().len(), 3);
/// assert_eq!(sample_size!(json!([1, 2, 3]), 0), json!([]));
/// ```
#[macro_export]
macro_rules! sample_size {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::sample_size($a, 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sample_size($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sample_size($a, $b)
    };
}

/// `_x` helper for [sample_size()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [sample_size()] and read the returned `Value`.
pub fn sample_size_x() {
    todo!()
}
/// Based on [sample_size_x()]
#[macro_export]
macro_rules! sample_size_x {
    ($($t:tt)*) => {
        $crate::sample_size_x()
    };
}

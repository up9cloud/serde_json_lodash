use crate::lib::Value;
use crate::array::sorted_index::sorted_index_impl;

/// See lodash [sortedIndexOf](https://lodash.com/docs/#sortedIndexOf)
///
/// Like `index_of`, but optimized for sorted arrays. Returns `-1` if not
/// found
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_index_of;
/// # use serde_json::json;
/// assert_eq!(sorted_index_of(json!([4, 5, 5, 5, 6]), json!(5)), 1);
/// ```
pub fn sorted_index_of(array: Value, value: Value) -> isize {
    let i = sorted_index_impl(&array, &value, false, |v| v.clone());
    if let Value::Array(vec) = &array
        && i < vec.len()
        && vec[i] == value
    {
        return i as isize;
    }
    -1
}

/// Based on [sorted_index_of()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_index_of!(json!([4, 5, 5, 5, 6]), json!(5)), 1);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_index_of!(), -1);
/// assert_eq!(sorted_index_of!(json!([1, 2, 3]), json!(9)), -1);
/// ```
#[macro_export]
macro_rules! sorted_index_of {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index_of($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_index_of($a, $b)
    };
}

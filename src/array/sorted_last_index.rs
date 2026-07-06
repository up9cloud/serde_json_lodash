use crate::lib::Value;
use crate::array::sorted_index::sorted_index_impl;

/// See lodash [sortedLastIndex](https://lodash.com/docs/#sortedLastIndex)
///
/// Returns the highest index at which `value` should be inserted to keep the
/// array sorted
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_last_index;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index(json!([4, 5, 5, 5, 6]), json!(5)), 4);
/// ```
pub fn sorted_last_index(array: Value, value: Value) -> usize {
    sorted_index_impl(&array, &value, true, |v| v.clone())
}

/// Based on [sorted_last_index()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_last_index!(json!([4, 5, 5, 5, 6]), json!(5)), 4);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index!(), 0);
/// assert_eq!(sorted_last_index!(json!([20, 30, 30, 50]), json!(30)), 3);
/// ```
#[macro_export]
macro_rules! sorted_last_index {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        0
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_last_index($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_last_index($a, $b)
    };
}

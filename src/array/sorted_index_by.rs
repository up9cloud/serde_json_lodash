use crate::lib::Value;
use crate::array::sorted_index::sorted_index_impl;

/// See lodash [sortedIndexBy](https://lodash.com/docs/#sortedIndexBy)
///
/// `iteratee` maps each element (and the value) to the sort key
pub fn sorted_index_by(array: Value, value: Value, iteratee: fn(&Value) -> Value) -> usize {
    sorted_index_impl(&array, &value, false, iteratee)
}

/// Based on [sorted_index_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([{ "x": 4 }, { "x": 5 }]);
/// assert_eq!(
///   sorted_index_by!(objects, json!({ "x": 4 }), |o| o["x"].clone()),
///   0
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_index_by!(), 0);
/// assert_eq!(sorted_index_by!(json!([30, 50]), json!(40)), 1);
/// ```
#[macro_export]
macro_rules! sorted_index_by {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        0
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::sorted_index_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::sorted_index_by($a, $b, $c)
    };
}

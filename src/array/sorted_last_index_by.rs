use crate::lib::Value;
use crate::array::sorted_index::sorted_index_impl;

/// See lodash [sortedLastIndexBy](https://lodash.com/docs/#sortedLastIndexBy)
///
/// `iteratee` maps each element (and the value) to the sort key
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_last_index_by;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_by(json!([4, 5, 5, 6]), json!(5), |v| v.clone()), 3);
/// ```
pub fn sorted_last_index_by(array: Value, value: Value, iteratee: fn(&Value) -> Value) -> usize {
    sorted_index_impl(&array, &value, true, iteratee)
}

/// Based on [sorted_last_index_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([{ "x": 4 }, { "x": 5 }, { "x": 5 }]);
/// assert_eq!(
///   sorted_last_index_by!(objects, json!({ "x": 5 }), |o| o["x"].clone()),
///   3
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_by!(), 0);
/// assert_eq!(sorted_last_index_by!(json!([4, 5, 5, 6]), json!(5)), 3);
/// ```
#[macro_export]
macro_rules! sorted_last_index_by {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        0
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_last_index($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::sorted_last_index_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::sorted_last_index_by($a, $b, $c)
    };
}

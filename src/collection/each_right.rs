use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [forEachRight](https://lodash.com/docs/#forEachRight)
///
/// Like [each()](fn@crate::each) but iterates from the end
pub fn each_right(collection: Value, iteratee: fn(&Value) -> bool) -> Value {
    for v in collection_values(&collection).iter().rev() {
        if !iteratee(v) {
            break;
        }
    }
    collection
}

/// Based on [each_right()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([1, 2]);
/// assert_eq!(each_right!(array.clone(), |n| { println!("{}", n); true }), array);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(each_right!(), json!(null));
/// assert_eq!(each_right!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! each_right {
    () => {
        serde_json::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::each_right($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::each_right($a, $b)
    };
}

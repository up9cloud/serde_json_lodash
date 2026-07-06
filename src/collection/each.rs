use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [forEach](https://lodash.com/docs/#forEach)
///
/// Invokes `iteratee` for each element; returning `false` stops iteration.
/// Returns `collection`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::each;
/// # use serde_json::json;
/// assert_eq!(each(json!([1, 2, 3]), |_| true), json!([1, 2, 3]));
/// ```
pub fn each(collection: Value, iteratee: fn(&Value) -> bool) -> Value {
    for v in collection_values(&collection) {
        if !iteratee(&v) {
            break;
        }
    }
    collection
}

/// Based on [each()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([1, 2]);
/// assert_eq!(each!(array.clone(), |n| { println!("{}", n); true }), array);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(each!(), json!(null));
/// assert_eq!(each!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! each {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::each($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::each($a, $b)
    };
}

/// `_x` helper for [each()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [each()] and read the returned `Value`.
pub fn each_x() {
    todo!()
}
/// Based on [each_x()]
#[macro_export]
macro_rules! each_x {
    ($($t:tt)*) => {
        $crate::each_x()
    };
}

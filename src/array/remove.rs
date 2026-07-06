use crate::lib::{json, Value};

/// See lodash [remove](https://lodash.com/docs/#remove)
///
/// Removes the elements matching `predicate` from `array` (mutating it) and
/// returns the removed elements
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::remove;
/// # use serde_json::json;
/// let mut a = json!([1, 2, 3, 4]);
/// assert_eq!(remove(&mut a, |n| n.as_i64().unwrap() % 2 == 0), json!([2, 4]));
/// ```
pub fn remove(array: &mut Value, predicate: fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => {
            let mut kept = vec![];
            let mut removed = vec![];
            for v in vec.drain(..) {
                if predicate(&v) {
                    removed.push(v);
                } else {
                    kept.push(v);
                }
            }
            *vec = kept;
            Value::Array(removed)
        }
        _ => json!([]),
    }
}

/// Based on [remove()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let mut array = json!([1, 2, 3, 4]);
/// let removed = remove!(&mut array, |n| n.as_i64().unwrap() % 2 == 0);
/// assert_eq!(array, json!([1, 3]));
/// assert_eq!(removed, json!([2, 4]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// let mut a = json!([]);
/// assert_eq!(remove!(&mut a), json!([]));
/// let mut b = json!([1, 2, 3]);
/// assert_eq!(remove!(&mut b, |_| false), json!([]));
/// assert_eq!(b, json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! remove {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::remove($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::remove($a, $b)
    };
}

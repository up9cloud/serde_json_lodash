use crate::lib::{json, Value};
use crate::array::zip::zip_arrays;

/// See lodash [unzip](https://lodash.com/docs/#unzip)
///
/// The inverse of `zip`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::unzip;
/// # use serde_json::json;
/// assert_eq!(unzip(json!([["a", 1, true], ["b", 2, false]])), json!([["a", "b"], [1, 2], [true, false]]));
/// ```
pub fn unzip(array: Value) -> Value {
    match array {
        Value::Array(groups) => zip_arrays(groups),
        _ => json!([]),
    }
}

/// Based on [unzip()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   unzip!(json!([["a", 1, true], ["b", 2, false]])),
///   json!([["a", "b"], [1, 2], [true, false]])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(unzip!(), json!([]));
/// assert_eq!(unzip!(json!([])), json!([]));
/// ```
#[macro_export]
macro_rules! unzip {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::unzip($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::unzip($a)
    };
}

use crate::lib::{Value, json};

use crate::array::zip::zip_arrays;

/// Fn form of [unzip!](crate::unzip!); see it for the full docs
///
/// `_x` form: **not provided** — see [unzip_x()]
///
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

/// See lodash [unzip](https://lodash.com/docs/#unzip)
///
/// The inverse of `zip`
///
/// Fn form: [unzip()] | `_x` form: **not provided** — see [unzip_x()]
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [unzip!](crate::unzip!) and read the returned `Value`.
///
/// Macro form: [unzip_x!](crate::unzip_x!)
pub fn unzip_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [unzip!](crate::unzip!) and read the returned `Value`.
///
/// Fn form: [unzip_x()]
#[macro_export]
macro_rules! unzip_x {
    ($($t:tt)*) => {
        $crate::unzip_x()
    };
}

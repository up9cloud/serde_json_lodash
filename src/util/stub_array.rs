use crate::lib::{Value, json};

/// Fn form of [stub_array!](crate::stub_array!); see it for the full docs
///
/// `_x` form: **not provided** — see [stub_array_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::stub_array;
/// # use serde_json::json;
/// assert_eq!(stub_array(), json!([]));
/// ```
pub fn stub_array() -> Value {
    json!([])
}

/// See lodash [stubArray](https://lodash.com/docs/#stubArray)
///
/// Fn form: [stub_array()] | `_x` form: **not provided** — see [stub_array_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(stub_array!(), json!([]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_array!(), json!([]));
/// ```
#[macro_export]
macro_rules! stub_array {
    () => {
        $crate::stub_array()
    };
}

build_not_provided_x!(stub_array, stub_array_x);

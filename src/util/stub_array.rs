use crate::lib::{json, Value};

/// See lodash [stubArray](https://lodash.com/docs/#stubArray)
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

/// Based on [stub_array()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(stub_array!(), json!([]));
/// ```
#[macro_export]
macro_rules! stub_array {
    () => {
        $crate::stub_array()
    };
}

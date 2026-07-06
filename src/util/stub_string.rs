use crate::lib::{json, Value};

/// See lodash [stubString](https://lodash.com/docs/#stubString)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::stub_string;
/// # use serde_json::json;
/// assert_eq!(stub_string(), json!(""));
/// ```
pub fn stub_string() -> Value {
    json!("")
}

/// Based on [stub_string()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(stub_string!(), json!(""));
/// ```
#[macro_export]
macro_rules! stub_string {
    () => {
        $crate::stub_string()
    };
}

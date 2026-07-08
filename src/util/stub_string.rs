use crate::lib::{Value, json};

/// Fn form of [stub_string!](crate::stub_string!); see it for the full docs
///
/// `_x` form: **not provided** — see [stub_string_x()]
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

/// See lodash [stubString](https://lodash.com/docs/#stubString)
///
/// Fn form: [stub_string()] | `_x` form: **not provided** — see [stub_string_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(stub_string!(), json!(""));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_string!(), json!(""));
/// ```
#[macro_export]
macro_rules! stub_string {
    () => {
        $crate::stub_string()
    };
}

build_not_provided_x!(stub_string, stub_string_x);

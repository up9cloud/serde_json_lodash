use crate::lib::{Value, json};

/// Fn form of [stub_object!](crate::stub_object!); see it for the full docs
///
/// `_x` form: **not provided** — see [stub_object_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::stub_object;
/// # use serde_json::json;
/// assert_eq!(stub_object(), json!({}));
/// ```
pub fn stub_object() -> Value {
    json!({})
}

/// See lodash [stubObject](https://lodash.com/docs/#stubObject)
///
/// Fn form: [stub_object()] | `_x` form: **not provided** — see [stub_object_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(stub_object!(), json!({}));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_object!(), json!({}));
/// ```
#[macro_export]
macro_rules! stub_object {
    () => {
        $crate::stub_object()
    };
}

build_not_provided_x!(stub_object, stub_object_x);

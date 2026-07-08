use crate::lib::{Value, json};

/// Fn form of [stub_true!](crate::stub_true!); see it for the full docs
///
/// `_x` form: **not provided** — see [stub_true_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::stub_true;
/// # use serde_json::json;
/// assert_eq!(stub_true(), json!(true));
/// ```
pub fn stub_true() -> Value {
    json!(true)
}

/// See lodash [stubTrue](https://lodash.com/docs/#stubTrue)
///
/// Fn form: [stub_true()] | `_x` form: **not provided** — see [stub_true_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(stub_true!(), json!(true));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_true!(), json!(true));
/// ```
#[macro_export]
macro_rules! stub_true {
    () => {
        $crate::stub_true()
    };
}

build_not_provided_x!(stub_true, stub_true_x);

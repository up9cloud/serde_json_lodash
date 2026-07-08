use crate::lib::{Value, json};

/// Fn form of [stub_false!](crate::stub_false!); see it for the full docs
///
/// `_x` form: **not provided** — see [stub_false_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::stub_false;
/// # use serde_json::json;
/// assert_eq!(stub_false(), json!(false));
/// ```
pub fn stub_false() -> Value {
    json!(false)
}

/// See lodash [stubFalse](https://lodash.com/docs/#stubFalse)
///
/// Fn form: [stub_false()] | `_x` form: **not provided** — see [stub_false_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(stub_false!(), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_false!(), json!(false));
/// ```
#[macro_export]
macro_rules! stub_false {
    () => {
        $crate::stub_false()
    };
}

build_not_provided_x!(stub_false, stub_false_x);

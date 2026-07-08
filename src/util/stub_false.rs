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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [stub_false!](crate::stub_false!) and read the returned
/// `Value`.
///
/// Macro form: [stub_false_x!](crate::stub_false_x!)
pub fn stub_false_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [stub_false!](crate::stub_false!) and read the returned
/// `Value`.
///
/// Fn form: [stub_false_x()]
#[macro_export]
macro_rules! stub_false_x {
    ($($t:tt)*) => {
        $crate::stub_false_x()
    };
}

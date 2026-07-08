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
#[macro_export]
macro_rules! stub_true {
    () => {
        $crate::stub_true()
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [stub_true!](crate::stub_true!) and read the returned
/// `Value`.
///
/// Macro form: [stub_true_x!](crate::stub_true_x!)
pub fn stub_true_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [stub_true!](crate::stub_true!) and read the returned
/// `Value`.
///
/// Fn form: [stub_true_x()]
#[macro_export]
macro_rules! stub_true_x {
    ($($t:tt)*) => {
        $crate::stub_true_x()
    };
}

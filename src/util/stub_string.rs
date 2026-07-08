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
#[macro_export]
macro_rules! stub_string {
    () => {
        $crate::stub_string()
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [stub_string!](crate::stub_string!) and read the
/// returned `Value`.
///
/// Macro form: [stub_string_x!](crate::stub_string_x!)
pub fn stub_string_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [stub_string!](crate::stub_string!) and read the
/// returned `Value`.
///
/// Fn form: [stub_string_x()]
#[macro_export]
macro_rules! stub_string_x {
    ($($t:tt)*) => {
        $crate::stub_string_x()
    };
}

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

/// `_x` helper for [stub_string()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [stub_string()] and read the returned `Value`.
pub fn stub_string_x() {
    todo!()
}
/// Based on [stub_string_x()]
#[macro_export]
macro_rules! stub_string_x {
    ($($t:tt)*) => {
        $crate::stub_string_x()
    };
}

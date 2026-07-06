use crate::lib::{json, Value};

/// See lodash [stubTrue](https://lodash.com/docs/#stubTrue)
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

/// Based on [stub_true()]
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

/// `_x` helper for [stub_true()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [stub_true()] and read the returned `Value`.
pub fn stub_true_x() {
    todo!()
}
/// Based on [stub_true_x()]
#[macro_export]
macro_rules! stub_true_x {
    ($($t:tt)*) => {
        $crate::stub_true_x()
    };
}

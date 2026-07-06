use crate::lib::{json, Value};

/// See lodash [stubFalse](https://lodash.com/docs/#stubFalse)
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

/// Based on [stub_false()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(stub_false!(), json!(false));
/// ```
#[macro_export]
macro_rules! stub_false {
    () => {
        $crate::stub_false()
    };
}

/// `_x` helper for [stub_false()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [stub_false()] and read the returned `Value`.
pub fn stub_false_x() {
    todo!()
}
/// Based on [stub_false_x()]
#[macro_export]
macro_rules! stub_false_x {
    ($($t:tt)*) => {
        $crate::stub_false_x()
    };
}

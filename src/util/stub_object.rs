use crate::lib::{json, Value};

/// See lodash [stubObject](https://lodash.com/docs/#stubObject)
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

/// Based on [stub_object()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(stub_object!(), json!({}));
/// ```
#[macro_export]
macro_rules! stub_object {
    () => {
        $crate::stub_object()
    };
}

/// `_x` helper for [stub_object()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [stub_object()] and read the returned `Value`.
pub fn stub_object_x() {
    todo!()
}

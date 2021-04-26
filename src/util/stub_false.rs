use crate::lib::{json, Value};

///
pub fn stub_false() -> Box<dyn Fn() -> Value> {
    Box::new(|| json!(false))
}

/// Description can be found in [lodash stubFalse](https://lodash.com/docs/#stubFalse)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // dynamic parameters, not implemented
/// //assert_eq!(
/// //  times!(2, stub_false!()),
/// //  json!([false, false])
/// //);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_false!()(), json!(false));
/// ```
#[macro_export]
macro_rules! stub_false {
    () => {
        $crate::stub_false()
    };
}

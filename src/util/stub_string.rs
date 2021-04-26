use crate::lib::{Value};

///
pub fn stub_string() -> Box<dyn Fn() -> Value> {
    Box::new(|| Value::String("".to_owned()))
}

/// Description can be found in [lodash stubString](https://lodash.com/docs/#stubString)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // dynamic parameters, not implemented
/// //assert_eq!(
/// //  times!(2, stub_string!()),
/// //  json!(["", ""])
/// //);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_string!()(), json!(""));
/// ```
#[macro_export]
macro_rules! stub_string {
    () => {
        $crate::stub_string()
    };
}

use crate::lib::{Value};

/// See lodash [stubTrue](https://lodash.com/docs/#stubTrue)
pub fn stub_true() -> Box<dyn Fn() -> Value> {
    Box::new(|| Value::Bool(true))
}

/// Based on [stub_true()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // dynamic parameters, not implemented
/// //assert_eq!(
/// //  times!(2, stub_true!()),
/// //  json!([true, true])
/// //);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_true!()(), json!(true));
/// ```
#[macro_export]
macro_rules! stub_true {
    () => {
        $crate::stub_true()
    };
}

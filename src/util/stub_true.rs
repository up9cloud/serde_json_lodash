use crate::lib::{Value};

///
pub fn stub_true() -> Box<dyn Fn() -> Value> {
    Box::new(|| Value::Bool(true))
}

/// Description can be found in [lodash stubTrue](https://lodash.com/docs/#stubTrue)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // conflict with dynamic parameters, no implement
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

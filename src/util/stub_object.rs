use crate::lib::{json, Value};

/// See lodash [stubObject](https://lodash.com/docs/#stubObject)
pub fn stub_object() -> Box<dyn Fn() -> Value> {
    Box::new(|| json!({}))
}

/// Based on [stub_object()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // dynamic parameters, not implemented
/// //let objects = times!(2, stub_object!());
/// //assert_eq!(
/// //  objects,
/// //  json!([{}, {}])
/// //);
/// //assert_ne!(
/// //  println("{:p}", objects[0]),
/// //  println("{:p}", objects[1])
/// //);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_object!()(), json!({}));
/// ```
#[macro_export]
macro_rules! stub_object {
    () => {
        $crate::stub_object()
    };
}

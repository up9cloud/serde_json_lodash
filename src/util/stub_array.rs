use crate::lib::{json, Value};

///
pub fn stub_array() -> Box<dyn Fn() -> Value> {
    Box::new(|| json!([]))
}

/// Description can be found in [lodash stubArray](https://lodash.com/docs/#stubArray)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // dynamic parameters, not implemented
/// //let arrays = times!(2, stub_array!());
/// //assert_eq!(
/// //  arrays,
/// //  json!([[], []])
/// //);
/// //assert_ne!(
/// //  println("{:p}", arrays[0]),
/// //  println("{:p}", arrays[1])
/// //);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(stub_array!()(), json!([]));
/// ```
#[macro_export]
macro_rules! stub_array {
    () => {
        $crate::stub_array()
    };
}

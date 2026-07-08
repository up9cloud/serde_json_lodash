use crate::lib::Value;

/// Fn form of [pull_all_with!](crate::pull_all_with!); see it for the full docs
///
/// `_x` form: **not provided** — see [pull_all_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pull_all_with;
/// # use serde_json::json;
/// assert_eq!(pull_all_with(json!([null,0]), json!([null]), |_, _| false), json!([null,0]));
/// ```
pub fn pull_all_with(
    mut array: Value,
    values: Value,
    comparator: impl Fn(&Value, &Value) -> bool,
) -> Value {
    let new_vec = match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return array;
        }
        Value::Array(ref vec) => {
            let values_vec = match values {
                Value::Null
                | Value::Bool(_)
                | Value::Number(_)
                | Value::String(_)
                | Value::Object(_) => return array,
                Value::Array(vec) => vec,
            };
            let mut new_vec = vec![];
            'a: for item in vec.iter() {
                for value in values_vec.iter() {
                    if comparator(item, value) {
                        continue 'a;
                    }
                }
                new_vec.push(item.clone())
            }
            new_vec
        }
    };
    *array.as_array_mut().unwrap() = new_vec;
    array
}

/// See lodash [pullAllWith](https://lodash.com/docs/#pullAllWith)
///
/// Fn form: [pull_all_with()] | `_x` form: **not provided** — see [pull_all_with_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([{ 'x': 1, 'y': 2 }, { 'x': 3, 'y': 4 }, { 'x': 5, 'y': 6 }]);
/// assert_eq!(
///   pull_all_with!(array, json!([{ 'x': 3, 'y': 4 }]), |a, b| a == b),
///   json!([{ 'x': 1, 'y': 2 }, { 'x': 5, 'y': 6 }])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pull_all_with!(), json!(null));
/// assert_eq!(pull_all_with!(json!(null)), json!(null));
/// assert_eq!(pull_all_with!(json!(false)), json!(false));
/// assert_eq!(pull_all_with!(json!(0)), json!(0));
/// assert_eq!(pull_all_with!(json!("")), json!(""));
/// assert_eq!(pull_all_with!(json!([])), json!([]));
/// assert_eq!(pull_all_with!(json!([[]]), json!([])), json!([[]]));
/// assert_eq!(pull_all_with!(json!([{}]), json!({})), json!([{}]));
/// assert_eq!(pull_all_with!(json!([null]), json!([null])), json!([]));
/// assert_eq!(pull_all_with!(json!([null,0]), json!([null]), |_, _| false), json!([null,0]));
/// assert_eq!(pull_all_with!(json!([null,0]), json!([null]), |a, _| a == 0), json!([null]));
/// assert_eq!(pull_all_with!(json!({})), json!({}));
/// ```
#[macro_export]
macro_rules! pull_all_with {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pull_all($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pull_all_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pull_all_with($a, $b, $c)
    };
}

build_not_provided_x!(pull_all_with, pull_all_with_x);

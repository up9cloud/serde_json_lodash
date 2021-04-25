use crate::lib::{Value};

///
pub fn pull_all_by(mut array: Value, values: Value, iteratee: fn(&Value) -> &Value) -> Value {
    let new_vec = match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return array
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
                let id_item = iteratee(item);
                for value in values_vec.iter() {
                    let id_value = iteratee(value);
                    if id_item == id_value {
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

/// Description can be found in [lodash pullAllBy](https://lodash.com/docs/#pullAllBy)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([{ 'x': 1 }, { 'x': 2 }, { 'x': 3 }, { 'x': 1 }]);
/// // conflict with fn, no implemented
/// // assert_eq!(
/// //   pull_all_by!(array, json!([{ 'x': 1 }, { 'x': 3 }]), "x"),
/// //   json!(0)
/// // );
/// assert_eq!(
///   pull_all_by!(array, json!([{ 'x': 1 }, { 'x': 3 }]), |o| &o["x"]),
///   json!([{ 'x': 2 }])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pull_all_by!(), json!(null));
/// assert_eq!(pull_all_by!(json!(null)), json!(null));
/// assert_eq!(pull_all_by!(json!(false)), json!(false));
/// assert_eq!(pull_all_by!(json!(0)), json!(0));
/// assert_eq!(pull_all_by!(json!("")), json!(""));
/// assert_eq!(pull_all_by!(json!([])), json!([]));
/// assert_eq!(pull_all_by!(json!([[]]), json!([])), json!([[]]));
/// assert_eq!(pull_all_by!(json!([{}]), json!({})), json!([{}]));
/// assert_eq!(pull_all_by!(json!([null]), json!([null])), json!([]));
/// assert_eq!(pull_all_by!(json!([null,0]), json!([null]), |x| &x), json!([0]));
/// assert_eq!(pull_all_by!(json!([null,0]), json!([null]), |x| &x["__non__"]), json!([]));
/// assert_eq!(pull_all_by!(json!({})), json!({}));
/// ```
#[macro_export]
macro_rules! pull_all_by {
    () => {
        json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pull_all($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::pull_all_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::pull_all_by($a, $b, $c)
    };
}

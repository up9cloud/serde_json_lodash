use crate::lib::{Value};

///
pub fn find_index(array: Value, predicate: fn(&Value) -> bool, from_index: usize) -> isize {
    match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return -1
        }
        Value::Array(vec) => {
            if vec.is_empty() {
                return -1;
            }
            if from_index >= vec.len() {
                return -1;
            }
            for (i, item) in vec.iter().enumerate().skip(from_index) {
                if predicate(item) {
                    return i as isize;
                }
            }
        }
    };
    -1
}

/// Description can be found in [lodash findIndex](https://lodash.com/docs/#findIndex)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "barney",  "active": false },
///   { "user": "fred",    "active": false },
///   { "user": "pebbles", "active": true }
/// ]);
/// assert_eq!(
///   find_index!(users, |o| o["user"] == "barney".to_string()),
///   json!(0)
/// );
/// // conflict with fn, no implemented
/// // assert_eq!(
/// //   find_index!(users, json!({ "user": "fred", "active": false })),
/// //   json!(1)
/// // );
/// // assert_eq!(
/// //   find_index!(users, json!(["active", false])),
/// //   json!(0)
/// // );
/// // assert_eq!(
/// //   find_index!(users, json!("active")),
/// //   json!(2)
/// // );
/// // assert_eq!(
/// //   find_index!(users, "active"),
/// //   json!(2)
/// // );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_index!(), -1);
/// assert_eq!(find_index!(json!(null)), -1);
/// assert_eq!(find_index!(json!(true)), -1);
/// assert_eq!(find_index!(json!(0)), -1);
/// assert_eq!(find_index!(json!("")), -1);
/// assert_eq!(find_index!(json!([])), -1);
/// assert_eq!(find_index!(json!([{"a":null},{"a":false},{"a":0},{"a":""},{"a":[]}]), |_| true), 0);
/// assert_eq!(find_index!(json!([{"a":null},{"a":false},{"a":0},{"a":""},{"a":[]}]), |_| true, 1), 1);
/// assert_eq!(find_index!(json!({})), -1);
/// # // assert_eq!(find_index!(json!([{"a":null},{"a":false},{"a":0},{"a":""},{"a":[]}]), 4));
/// # // assert_eq!(find_index!(json!([null,false,0,"",{}]), json!(null)), 4);
/// # // assert_eq!(find_index!(json!([null,false,0,"",{}]), json!(false)), -1);
/// # // assert_eq!(find_index!(json!([null,false,0,"",{}]), json!(0)), -1);
/// # // assert_eq!(find_index!(json!([null,false,1,"",{}]), json!(1)), -1);
/// # // assert_eq!(find_index!(json!([null,false,0,"",{}]), json!("")), -1);
/// # // assert_eq!(find_index!(json!([null,false,0,"",{}]), json!([])), -1);
/// # // assert_eq!(find_index!(json!([null,false,0,"",{}]), json!({})), 0);
/// # // assert_eq!(find_index!(json!([null,false,0,"",{}]), json!({"a":1})), -1);
/// # // assert_eq!(find_index!(json!([null,false,0,"",{"a":1}]), json!({"a":1})), 4);
/// # // assert_eq!(find_index!(json!([null,false,0,"",{"a":1}]), json!({"a":2})), -1);
/// ```
#[macro_export]
macro_rules! find_index {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find_index($a, $b, 0)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::find_index($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::find_index($a, $b, $c)
    };
}

use crate::lib::{Value};

///
pub fn find_last_index(array: Value, predicate: fn(&Value) -> bool, from_index: usize) -> isize {
    match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return -1
        }
        Value::Array(ref vec) => {
            if vec.is_empty() {
                return -1;
            }
            let mut real_from_index = from_index;
            if from_index >= vec.len() {
                real_from_index = vec.len() - 1;
            }
            for i in (0..=real_from_index).rev() {
                if predicate(&vec[i]) {
                    return i as isize;
                }
            }
        }
    };
    -1
}

/// See [lodash findLastIndex](https://lodash.com/docs/#findLastIndex)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "barney",  "active": true },
///   { "user": "fred",    "active": false },
///   { "user": "pebbles", "active": false }
/// ]);
/// assert_eq!(
///   find_last_index!(users, |o| o["user"] == "pebbles".to_string()),
///   json!(2)
/// );
/// // conflict with fn, no implemented
/// // assert_eq!(
/// //   find_last_index!(users, json!({ "user": "barney", "active": true })),
/// //   json!(0)
/// // );
/// // assert_eq!(
/// //   find_last_index!(users, json!(["active", false])),
/// //   json!(2)
/// // );
/// // assert_eq!(
/// //   find_last_index!(users, json!("active")),
/// //   json!(0)
/// // );
/// // assert_eq!(
/// //   find_last_index!(users, "active"),
/// //   json!(0)
/// // );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_last_index!(), -1);
/// assert_eq!(find_last_index!(json!(null)), -1);
/// assert_eq!(find_last_index!(json!(true)), -1);
/// assert_eq!(find_last_index!(json!(0)), -1);
/// assert_eq!(find_last_index!(json!("")), -1);
/// assert_eq!(find_last_index!(json!([])), -1);
/// assert_eq!(find_last_index!(json!([{"a":null},{"a":false},{"a":0},{"a":""},{"a":[]}]), |_| true), 4);
/// assert_eq!(find_last_index!(json!([{"a":null},{"a":false},{"a":0},{"a":""},{"a":[]}]), |_| true, 1), 1);
/// assert_eq!(find_last_index!(json!([{"a":null},{"a":false},{"a":0},{"a":""},{"a":[]}]), |x| x == 0, 1), -1);
/// assert_eq!(find_last_index!(json!({})), -1);
/// # // assert_eq!(find_last_index!(json!([{"a":null},{"a":false},{"a":0},{"a":""},{"a":[]}]), 4));
/// # // assert_eq!(find_last_index!(json!([null,false,0,"",{}]), json!(null)), 4);
/// # // assert_eq!(find_last_index!(json!([null,false,0,"",{}]), json!(false)), -1);
/// # // assert_eq!(find_last_index!(json!([null,false,0,"",{}]), json!(0)), -1);
/// # // assert_eq!(find_last_index!(json!([null,false,1,"",{}]), json!(1)), -1);
/// # // assert_eq!(find_last_index!(json!([null,false,0,"",{}]), json!("")), -1);
/// # // assert_eq!(find_last_index!(json!([null,false,0,"",{}]), json!([])), -1);
/// # // assert_eq!(find_last_index!(json!([null,false,0,"",{}]), json!({})), 0);
/// # // assert_eq!(find_last_index!(json!([null,false,0,"",{}]), json!({"a":1})), -1);
/// # // assert_eq!(find_last_index!(json!([null,false,0,"",{"a":1}]), json!({"a":1})), 4);
/// # // assert_eq!(find_last_index!(json!([null,false,0,"",{"a":1}]), json!({"a":2})), -1);
/// ```
#[macro_export]
macro_rules! find_last_index {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {{
        let from_index = $a.as_array().unwrap_or(&vec![]).len() - 1;
        $crate::find_last_index($a, $b, from_index)
    }};
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::find_last_index($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::find_last_index($a, $b, $c)
    };
}

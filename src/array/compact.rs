use crate::lib::{json, Value};

///
pub fn compact(v: Value) -> Value {
    match v {
        Value::Null => json!([]),
        Value::Bool(_) => json!([]),
        Value::Number(_) => json!([]),
        Value::String(s) => {
            let vec: Vec<Value> = s.chars().map(|v| Value::String(v.to_string())).collect();
            Value::Array(vec)
        }
        Value::Array(vec) => {
            let result = vec
                .into_iter()
                .filter(|v| match v {
                    Value::Null => false,
                    Value::Bool(b) => *b,
                    Value::Number(n) => match n.as_i64() {
                        Some(v) => v != 0,
                        None => false,
                    },
                    Value::String(s) => !s.is_empty(),
                    Value::Array(_) => true,
                    Value::Object(_) => true,
                })
                .collect::<Vec<_>>();
            Value::Array(result)
        }
        Value::Object(_) => json!([]),
    }
}

/// See [lodash compact](https://lodash.com/docs/#compact)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   compact!(json!([0, 1, false, 2, "", 3])),
///   json!([1, 2, 3])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(compact!(), json!([]));
/// assert_eq!(compact!(json!(null)), json!([]));
/// assert_eq!(compact!(json!(false)), json!([]));
/// assert_eq!(compact!(json!(-1.1)), json!([]));
/// assert_eq!(compact!(json!("ab")), json!(["a","b"]));
/// assert_eq!(compact!(json!({"a":1})), json!([]));
/// assert_eq!(compact!(json!([["ab"],{"a":1}])), json!([ [ "ab" ], { "a": 1 } ]));
/// ```
#[macro_export]
macro_rules! compact {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::compact($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::compact($a)
    };
}

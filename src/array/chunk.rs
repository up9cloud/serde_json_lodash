use crate::lib::{json, Value};

///
pub fn chunk(v: Value, size: usize) -> Value {
    if size == 0 {
        return json!([]);
    }
    match v {
        Value::Null => json!([]),
        Value::Bool(_) => json!([]),
        Value::Number(_) => json!([]),
        Value::String(s) => {
            if size == 1 {
                let vec: Vec<Value> = s.chars().map(|v| Value::String(v.to_string())).collect();
                return Value::Array(vec);
            }
            let chars: Vec<char> = s.chars().collect();
            let vec = &chars
                .chunks(size)
                .map(|chunk| chunk.iter().map(|c| Value::String(c.to_string())).collect())
                .collect::<Vec<Value>>();
            Value::Array(vec.to_owned())
        }
        Value::Array(vec) => {
            let result = vec
                .chunks(size)
                .map(|chunk| chunk.iter().map(|item| item.to_owned()).collect())
                .collect::<Vec<Value>>();
            Value::Array(result)
        }
        Value::Object(_) => json!([]),
    }
}

/// See [lodash chunk](https://lodash.com/docs/#chunk)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   chunk!(json!(['a', 'b', 'c', 'd']), 2),
///   json!([['a', 'b'], ['c', 'd']])
/// );
/// assert_eq!(
///   chunk!(json!(['a', 'b', 'c', 'd']), 3),
///   json!([['a', 'b', 'c'], ['d']])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(chunk!(), json!([]));
/// assert_eq!(chunk!(json!([1,2,3]), 0), json!([]));
/// assert_eq!(chunk!(json!(true)), json!([]));
/// assert_eq!(chunk!(json!(1)), json!([]));
/// assert_eq!(chunk!(json!("abc")), json!(["a","b","c"]));
/// assert_eq!(chunk!(json!("abc"), 2), json!([["a","b"],["c"]]));
/// assert_eq!(chunk!(json!("a世界"), 2), json!([["a","世"],["界"]]));
/// assert_eq!(chunk!(json!([true, 1, 'a', {}, []]), 2), json!([ [ true, 1 ], [ 'a', {} ], [ [] ] ]));
/// assert_eq!(chunk!(json!({})), json!([]));
/// ```
#[macro_export]
macro_rules! chunk {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::chunk($a, 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::chunk($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::chunk($a, $b)
    };
}

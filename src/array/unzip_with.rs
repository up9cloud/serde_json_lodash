use crate::lib::{json, Value};

/// See lodash [unzipWith](https://lodash.com/docs/#unzipWith)
///
/// The inverse of `zip_with`; `iteratee` combines each regrouped tuple
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::unzip_with;
/// # use serde_json::json;
/// assert_eq!(unzip_with(json!([[1, 10], [2, 20]]), |g| json!(g[0].as_i64().unwrap() + g[1].as_i64().unwrap())), json!([3, 30]));
/// ```
pub fn unzip_with(array: Value, iteratee: fn(&Value) -> Value) -> Value {
    let groups: Vec<Vec<Value>> = match array {
        Value::Array(outer) => outer
            .into_iter()
            .filter_map(|a| match a {
                Value::Array(v) => Some(v),
                _ => None,
            })
            .collect(),
        _ => return json!([]),
    };
    let max_len = groups.iter().map(|g| g.len()).max().unwrap_or(0);
    let mut out = vec![];
    for i in 0..max_len {
        let tuple: Vec<Value> = groups
            .iter()
            .map(|g| g.get(i).cloned().unwrap_or(Value::Null))
            .collect();
        out.push(iteratee(&Value::Array(tuple)));
    }
    Value::Array(out)
}

/// Based on [unzip_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   unzip_with!(json!([[1, 10], [2, 20]]), |g| json!(g[0].as_i64().unwrap() + g[1].as_i64().unwrap())),
///   json!([3, 30])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(unzip_with!(), json!([]));
/// assert_eq!(unzip_with!(json!([[1, 2], [3, 4]])), json!([[1, 3], [2, 4]]));
/// ```
#[macro_export]
macro_rules! unzip_with {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::unzip($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::unzip_with($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::unzip_with($a, $b)
    };
}

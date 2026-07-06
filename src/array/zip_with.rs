use crate::lib::Value;

/// See lodash [zipWith](https://lodash.com/docs/#zipWith)
///
/// `iteratee` combines the grouped values; it receives an array of one
/// element from each input array
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::zip_with;
/// # use serde_json::json;
/// assert_eq!(zip_with(json!([1, 2]), json!([10, 20]), |g| json!(g[0].as_i64().unwrap() + g[1].as_i64().unwrap())), json!([11, 22]));
/// ```
pub fn zip_with(array: Value, other: Value, iteratee: fn(&Value) -> Value) -> Value {
    let groups: Vec<Vec<Value>> = [array, other]
        .into_iter()
        .filter_map(|a| match a {
            Value::Array(v) => Some(v),
            _ => None,
        })
        .collect();
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

/// Based on [zip_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   zip_with!(json!([1, 2]), json!([10, 20]), |g| json!(g[0].as_i64().unwrap() + g[1].as_i64().unwrap())),
///   json!([11, 22])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(zip_with!(), json!([]));
/// assert_eq!(zip_with!(json!([1, 2]), json!([3, 4])), json!([[1, 3], [2, 4]]));
/// ```
#[macro_export]
macro_rules! zip_with {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::zip($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::zip_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::zip_with($a, $b, $c)
    };
}

/// `_x` helper for [zip_with()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [zip_with()] and read the returned `Value`.
pub fn zip_with_x() {
    todo!()
}

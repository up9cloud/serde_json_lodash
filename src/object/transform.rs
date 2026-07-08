use crate::lib::Value;

/// Fn form of [transform!](crate::transform!); see it for the full docs
///
/// `_x` form: **not provided** — see [transform_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::transform;
/// # use serde_json::json;
/// assert_eq!(transform(json!([2, 3, 4]), |mut acc, n, _k| {     acc.as_array_mut().unwrap().push(json!(n.as_i64().unwrap() * n.as_i64().unwrap()));     (acc, true)   }, json!([])), json!([4, 9, 16]));
/// ```
pub fn transform(
    collection: Value,
    iteratee: impl Fn(Value, &Value, &str) -> (Value, bool),
    accumulator: Value,
) -> Value {
    let mut acc = accumulator;
    match collection {
        Value::Array(vec) => {
            for (i, v) in vec.iter().enumerate() {
                let (next, keep) = iteratee(acc, v, &i.to_string());
                acc = next;
                if !keep {
                    break;
                }
            }
        }
        Value::Object(o) => {
            for (k, v) in o.iter() {
                let (next, keep) = iteratee(acc, v, k);
                acc = next;
                if !keep {
                    break;
                }
            }
        }
        _ => {}
    }
    acc
}

/// See lodash [transform](https://lodash.com/docs/#transform)
///
/// `iteratee` receives `(accumulator, value, key)` and returns
/// `(next_accumulator, keep_going)`; iteration stops when `keep_going` is
/// `false`. For arrays the key is the stringified index
///
/// Fn form: [transform()] | `_x` form: **not provided** — see [transform_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   transform!(json!([2, 3, 4]), |mut acc, n, _k| {
///     acc.as_array_mut().unwrap().push(json!(n.as_i64().unwrap() * n.as_i64().unwrap()));
///     (acc, true)
///   }, json!([])),
///   json!([4, 9, 16])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(transform!(), json!(null));
/// assert_eq!(transform!(json!([1, 2, 3])), json!([1, 2, 3]));
/// // stop early by returning false
/// assert_eq!(
///   transform!(json!([1, 2, 3]), |mut acc, n, _k| {
///     acc.as_array_mut().unwrap().push(n.clone());
///     (acc, n.as_i64().unwrap() < 2)
///   }, json!([])),
///   json!([1, 2])
/// );
/// ```
#[macro_export]
macro_rules! transform {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::transform($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::transform($a, $b, $c)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [transform!](crate::transform!) and read the returned
/// `Value`.
///
/// Macro form: [transform_x!](crate::transform_x!)
pub fn transform_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [transform!](crate::transform!) and read the returned
/// `Value`.
///
/// Fn form: [transform_x()]
#[macro_export]
macro_rules! transform_x {
    ($($t:tt)*) => {
        $crate::transform_x()
    };
}

use crate::lib::Value;

use crate::internal::value_to_option_number;

/// Fn form of [max!](crate::max!); see it for the full docs
///
/// `_x` form: **not provided** — see [max_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::max;
/// # use serde_json::json;
/// assert_eq!(max(json!([4, 2, 8, 6])), json!(8));
/// ```
pub fn max(array: Value) -> Value {
    match array {
        Value::Array(vec) => vec
            .into_iter()
            .filter(|v| {
                value_to_option_number(v.clone())
                    .and_then(|n| n.as_f64())
                    .is_some()
            })
            .fold(None, |acc: Option<Value>, v| match acc {
                None => Some(v),
                Some(cur) => {
                    let a = value_to_option_number(cur.clone())
                        .unwrap()
                        .as_f64()
                        .unwrap();
                    let b = value_to_option_number(v.clone()).unwrap().as_f64().unwrap();
                    if b > a { Some(v) } else { Some(cur) }
                }
            })
            .unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

/// See lodash [max](https://lodash.com/docs/#max)
///
/// Returns `Value::Null` for an empty array (js `undefined`)
///
/// Fn form: [max()] | `_x` form: **not provided** — see [max_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(max!(json!([4, 2, 8, 6])), json!(8));
/// assert_eq!(max!(json!([])), json!(null));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(max!(), json!(null));
/// assert_eq!(max!(json!(null)), json!(null));
/// assert_eq!(max!(json!([1, "3", 2])), json!("3")); // string coerces to number
/// ```
#[macro_export]
macro_rules! max {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::max($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::max($a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [max!](crate::max!) and read the returned `Value`.
///
/// Macro form: [max_x!](crate::max_x!)
pub fn max_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [max!](crate::max!) and read the returned `Value`.
///
/// Fn form: [max_x()]
#[macro_export]
macro_rules! max_x {
    ($($t:tt)*) => {
        $crate::max_x()
    };
}

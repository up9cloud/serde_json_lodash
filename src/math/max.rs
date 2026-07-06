use crate::lib::Value;
use crate::internal::value_to_option_number;

/// See lodash [max](https://lodash.com/docs/#max)
///
/// Returns `Value::Null` for an empty array (js `undefined`)
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

/// Based on [max()]
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

/// `_x` helper for [max()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [max()] and read the returned `Value`.
pub fn max_x() {
    todo!()
}

use crate::lib::Value;
use crate::internal::value_to_option_number;

/// See lodash [min](https://lodash.com/docs/#min)
///
/// Returns `Value::Null` for an empty array (js `undefined`)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::min;
/// # use serde_json::json;
/// assert_eq!(min(json!([4, 2, 8, 6])), json!(2));
/// ```
pub fn min(array: Value) -> Value {
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
                    if b < a { Some(v) } else { Some(cur) }
                }
            })
            .unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

/// Based on [min()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(min!(json!([4, 2, 8, 6])), json!(2));
/// assert_eq!(min!(json!([])), json!(null));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(min!(), json!(null));
/// assert_eq!(min!(json!(null)), json!(null));
/// ```
#[macro_export]
macro_rules! min {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::min($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::min($a)
    };
}

/// `_x` helper for [min()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [min()] and read the returned `Value`.
pub fn min_x() {
    todo!()
}

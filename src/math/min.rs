use crate::lib::Value;

use crate::internal::value_to_option_number_ref;

/// Fn form of [min!](crate::min!); see it for the full docs
///
/// `_x` form: **not provided** — see [min_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::min;
/// # use serde_json::json;
/// assert_eq!(min(json!([4, 2, 8, 6])), json!(2));
/// ```
pub fn min(array: Value) -> Value {
    match array {
        // coerce each element once, carrying the numeric key with the value
        Value::Array(vec) => vec
            .into_iter()
            .filter_map(|v| {
                value_to_option_number_ref(&v)
                    .and_then(|n| n.as_f64())
                    .map(|f| (f, v))
            })
            .fold(None, |acc: Option<(f64, Value)>, (f, v)| match acc {
                None => Some((f, v)),
                Some((fa, va)) => {
                    if f < fa {
                        Some((f, v))
                    } else {
                        Some((fa, va))
                    }
                }
            })
            .map(|(_, v)| v)
            .unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

/// See lodash [min](https://lodash.com/docs/#min)
///
/// Returns `Value::Null` for an empty array (js `undefined`)
///
/// Fn form: [min()] | `_x` form: **not provided** — see [min_x()]
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

build_not_provided_x!(min, min_x);

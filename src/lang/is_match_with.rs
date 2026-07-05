use crate::lib::Value;
use crate::internal::base_is_match;

/// See lodash [isMatchWith](https://lodash.com/docs/#isMatchWith)
///
/// The customizer is invoked for every property of `source` (top level
/// only); returning `None` falls back to the [is_match()](fn@crate::is_match) behavior
pub fn is_match_with(
    object: &Value,
    source: &Value,
    customizer: fn(&Value, &Value) -> Option<bool>,
) -> bool {
    match source {
        Value::Object(so) => match object {
            Value::Object(oo) => so.iter().all(|(k, sv)| match oo.get(k) {
                Some(ov) => match customizer(ov, sv) {
                    Some(result) => result,
                    None => base_is_match(ov, sv),
                },
                None => false,
            }),
            _ => so.is_empty(),
        },
        _ => match customizer(object, source) {
            Some(result) => result,
            None => base_is_match(object, source),
        },
    }
}

/// Based on [is_match_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// use serde_json::Value;
/// fn customizer(a: &Value, b: &Value) -> Option<bool> {
///   match (a.as_str(), b.as_str()) {
///     (Some(a), Some(b)) => Some(a.to_lowercase() == b.to_lowercase()),
///     _ => None,
///   }
/// }
/// let object = json!({ "greeting": "Hello" });
/// let source = json!({ "greeting": "hello" });
/// assert_eq!(is_match_with!(&object, &source, customizer), true);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_match_with!(), true);
/// assert_eq!(is_match_with!(&json!({"a": 1})), true);
/// assert_eq!(is_match_with!(&json!({"a": 1}), &json!({"a": 1})), true);
/// ```
#[macro_export]
macro_rules! is_match_with {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        true
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_match($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::is_match_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::is_match_with($a, $b, $c)
    };
}

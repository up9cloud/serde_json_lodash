use crate::lib::Value;
use crate::merge;

/// See lodash [mergeWith](https://lodash.com/docs/#mergeWith)
///
/// Like [merge()](fn@crate::merge), but `customizer(obj_value, src_value)` is
/// consulted for each top level key; returning `None` falls back to the
/// default recursive merge
pub fn merge_with(
    object: Value,
    source: Value,
    customizer: fn(&Value, &Value) -> Option<Value>,
) -> Value {
    match (object, source) {
        (Value::Object(mut o), Value::Object(s)) => {
            for (k, sv) in s {
                let ov = o.get(&k).cloned().unwrap_or(Value::Null);
                let value = match customizer(&ov, &sv) {
                    Some(v) => v,
                    None => merge(ov, sv),
                };
                o.insert(k, value);
            }
            Value::Object(o)
        }
        (o, _) => o,
    }
}

/// Based on [merge_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// use serde_json::Value;
/// // concatenate arrays instead of merging them by index
/// fn customizer(obj_value: &Value, src_value: &Value) -> Option<Value> {
///   if let (Value::Array(a), Value::Array(b)) = (obj_value, src_value) {
///     let mut v = a.clone();
///     v.extend(b.clone());
///     Some(Value::Array(v))
///   } else {
///     None
///   }
/// }
/// assert_eq!(
///   merge_with!(json!({ "a": [1] }), json!({ "a": [2] }), customizer),
///   json!({ "a": [1, 2] })
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(merge_with!(), json!(null));
/// assert_eq!(merge_with!(json!({"a": 1})), json!({"a": 1}));
/// assert_eq!(merge_with!(json!({"a": 1}), json!({"b": 2})), json!({"a": 1, "b": 2}));
/// ```
#[macro_export]
macro_rules! merge_with {
    () => {
        json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::merge($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::merge_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::merge_with($a, $b, $c)
    };
}

use crate::lib::{json, Value};

/// See lodash [assignWith](https://lodash.com/docs/#assignWith)
///
/// `customizer(obj_value, src_value)` produces the assigned value; returning
/// `None` falls back to the source value
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::assign_with;
/// # use serde_json::json;
/// assert_eq!(assign_with(json!({"a": 1}), json!({"b": 2}), |_o, _s| None), json!({"a": 1, "b": 2}));
/// ```
pub fn assign_with(
    object: Value,
    source: Value,
    customizer: fn(&Value, &Value) -> Option<Value>,
) -> Value {
    match (object, source) {
        (Value::Object(mut o), Value::Object(s)) => {
            for (k, sv) in s {
                let ov = o.get(&k).cloned().unwrap_or(Value::Null);
                let value = customizer(&ov, &sv).unwrap_or(sv);
                o.insert(k, value);
            }
            Value::Object(o)
        }
        (Value::Object(o), _) => Value::Object(o),
        _ => json!({}),
    }
}

/// Based on [assign_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// use serde_json::Value;
/// // keep the existing value when the source value is null
/// fn customizer(obj_value: &Value, src_value: &Value) -> Option<Value> {
///   if src_value.is_null() { Some(obj_value.clone()) } else { None }
/// }
/// assert_eq!(
///   assign_with!(json!({ "a": 1, "b": 2 }), json!({ "a": null, "c": 3 }), customizer),
///   json!({ "a": 1, "b": 2, "c": 3 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(assign_with!(), json!({}));
/// assert_eq!(assign_with!(json!({"a": 1})), json!({"a": 1}));
/// assert_eq!(assign_with!(json!({"a": 1}), json!({"b": 2})), json!({"a": 1, "b": 2}));
/// ```
#[macro_export]
macro_rules! assign_with {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::to_plain_object($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::assign($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::assign_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::assign_with($a, $b, $c)
    };
}

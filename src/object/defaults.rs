use crate::lib::{json, Value};

/// See lodash [defaults](https://lodash.com/docs/#defaults)
///
/// Assigns `source` properties for keys that resolve to `undefined`/missing
/// in `object`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::defaults;
/// # use serde_json::json;
/// assert_eq!(defaults(json!({"a": null}), json!({"a": 5})), json!({"a": 5}));
/// ```
pub fn defaults(object: Value, source: Value) -> Value {
    match (object, source) {
        (Value::Object(mut o), Value::Object(s)) => {
            for (k, v) in s {
                let missing = match o.get(&k) {
                    None => true,
                    Some(existing) => existing.is_null(),
                };
                if missing {
                    o.insert(k, v);
                }
            }
            Value::Object(o)
        }
        (Value::Object(o), _) => Value::Object(o),
        (_, Value::Object(s)) => Value::Object(s),
        _ => json!({}),
    }
}

/// Based on [defaults()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   defaults!(json!({ "a": 1 }), json!({ "b": 2 }), json!({ "a": 3, "c": 4 })),
///   json!({ "a": 1, "b": 2, "c": 4 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(defaults!(), json!({}));
/// assert_eq!(defaults!(json!({"a": 1})), json!({"a": 1}));
/// assert_eq!(defaults!(json!({"a": null}), json!({"a": 5})), json!({"a": 5}));
/// ```
#[macro_export]
macro_rules! defaults {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::to_plain_object($a)
    };
    ($a:expr, $($b:expr),+ $(,)*) => {{
        let mut acc = $crate::to_plain_object($a);
        $(
            acc = $crate::defaults(acc, $b);
        )+
        acc
    }};
}

/// `_x` helper for [defaults()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [defaults()] and read the returned `Value`.
pub fn defaults_x() {
    todo!()
}

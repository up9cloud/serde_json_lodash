use crate::lib::Value;

fn deep_default(object: Value, source: Value) -> Value {
    match (object, source) {
        (Value::Object(mut o), Value::Object(s)) => {
            for (k, sv) in s {
                match o.remove(&k) {
                    None => {
                        o.insert(k, sv);
                    }
                    Some(ov) if ov.is_null() => {
                        o.insert(k, sv);
                    }
                    Some(ov) => {
                        o.insert(k, deep_default(ov, sv));
                    }
                }
            }
            Value::Object(o)
        }
        (o, _) => o,
    }
}

/// Fn form of [defaults_deep!](crate::defaults_deep!); see it for the full docs
///
/// `_x` form: **not provided** — see [defaults_deep_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::defaults_deep;
/// # use serde_json::json;
/// assert_eq!(defaults_deep(json!({ "a": { "b": 2 } }), json!({ "a": { "b": 3, "c": 3 } })), json!({ "a": { "b": 2, "c": 3 } }));
/// ```
pub fn defaults_deep(object: Value, source: Value) -> Value {
    deep_default(object, source)
}

/// See lodash [defaultsDeep](https://lodash.com/docs/#defaultsDeep)
///
/// Fn form: [defaults_deep()] | `_x` form: **not provided** — see [defaults_deep_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   defaults_deep!(json!({ "a": { "b": 2 } }), json!({ "a": { "b": 3, "c": 3 } })),
///   json!({ "a": { "b": 2, "c": 3 } })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(defaults_deep!(), json!({}));
/// assert_eq!(defaults_deep!(json!({"a": 1})), json!({"a": 1}));
/// ```
#[macro_export]
macro_rules! defaults_deep {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::to_plain_object($a)
    };
    ($a:expr, $($b:expr),+ $(,)*) => {{
        let mut acc = $crate::to_plain_object($a);
        $(
            acc = $crate::defaults_deep(acc, $b);
        )+
        acc
    }};
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [defaults_deep!](crate::defaults_deep!) and read the
/// returned `Value`.
///
/// Macro form: [defaults_deep_x!](crate::defaults_deep_x!)
pub fn defaults_deep_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [defaults_deep!](crate::defaults_deep!) and read the
/// returned `Value`.
///
/// Fn form: [defaults_deep_x()]
#[macro_export]
macro_rules! defaults_deep_x {
    ($($t:tt)*) => {
        $crate::defaults_deep_x()
    };
}

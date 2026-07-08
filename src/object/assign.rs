use crate::lib::{Value, json};

/// Fn form of [assign!](crate::assign!); see it for the full docs
///
/// `_x` form: **not provided** — see [assign_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::assign;
/// # use serde_json::json;
/// assert_eq!(assign(json!({"a": 1}), json!({"a": 2})), json!({"a": 2}));
/// ```
pub fn assign(object: Value, source: Value) -> Value {
    match (object, source) {
        (Value::Object(mut o), Value::Object(s)) => {
            for (k, v) in s {
                o.insert(k, v);
            }
            Value::Object(o)
        }
        (Value::Object(o), _) => Value::Object(o),
        (_, Value::Object(s)) => Value::Object(s),
        _ => json!({}),
    }
}

/// See lodash [assign](https://lodash.com/docs/#assign)
///
/// Copies the own enumerable properties of `source` onto `object` (shallow)
///
/// Fn form: [assign()] | `_x` form: **not provided** — see [assign_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   assign!(json!({ "a": 0 }), json!({ "b": 1 }), json!({ "c": 2 })),
///   json!({ "a": 0, "b": 1, "c": 2 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(assign!(), json!({}));
/// assert_eq!(assign!(json!({"a": 1})), json!({"a": 1}));
/// assert_eq!(assign!(json!({"a": 1}), json!({"a": 2})), json!({"a": 2}));
/// ```
#[macro_export]
macro_rules! assign {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::to_plain_object($a)
    };
    ($a:expr, $($b:expr),+ $(,)*) => {{
        let mut acc = $crate::to_plain_object($a);
        $(
            acc = $crate::assign(acc, $b);
        )+
        acc
    }};
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [assign!](crate::assign!) and read the returned
/// `Value`.
///
/// Macro form: [assign_x!](crate::assign_x!)
pub fn assign_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [assign!](crate::assign!) and read the returned
/// `Value`.
///
/// Fn form: [assign_x()]
#[macro_export]
macro_rules! assign_x {
    ($($t:tt)*) => {
        $crate::assign_x()
    };
}

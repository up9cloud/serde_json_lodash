use crate::lib::{Map, Value};

// Both helpers consume their inputs so the recursion never clones: object
// subtrees are taken out with mem::take and source values are moved in.
fn merge_2_array(object_vec: Vec<Value>, source_vec: Vec<Value>) -> Value {
    if object_vec.is_empty() {
        return Value::Array(source_vec);
    }
    if source_vec.is_empty() {
        return Value::Array(object_vec);
    }
    let mut object_it = object_vec.into_iter();
    let mut new_v = vec![];
    for v in source_vec {
        new_v.push(match (object_it.next(), v) {
            (Some(Value::Object(om)), Value::Object(sm)) => merge_2_object(om, sm),
            (Some(Value::Array(oa)), Value::Array(sa)) => merge_2_array(oa, sa),
            (_, v) => v,
        });
    }
    Value::Array(new_v)
}

fn merge_2_object(mut object: Map<String, Value>, source: Map<String, Value>) -> Value {
    for (k, sv) in source {
        match object.get_mut(&k) {
            Some(ov) => {
                let owned = std::mem::take(ov);
                *ov = match (owned, sv) {
                    (Value::Null, sv) => {
                        if sv.is_null() {
                            Value::Null
                        } else {
                            sv
                        }
                    }
                    (Value::Array(oa), Value::Array(sa)) => merge_2_array(oa, sa),
                    (Value::Object(om), Value::Object(sm)) => merge_2_object(om, sm),
                    (_, sv) => sv,
                };
            }
            None => {
                object.insert(k, sv);
            }
        }
    }
    Value::Object(object)
}

/// Fn form of [merge!](crate::merge!); see it for the full docs
///
/// `_x` form: **not provided** — see [merge_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::merge;
/// # use serde_json::json;
/// assert_eq!(merge(json!({"a": 1}), json!({"b": 2})), json!({"a": 1, "b": 2}));
/// ```
pub fn merge(object: Value, source: Value) -> Value {
    match (object, source) {
        (Value::Object(o), Value::Object(s)) => merge_2_object(o, s),
        (Value::Array(o), Value::Array(s)) => merge_2_array(o, s),
        // TODO:
        // object with array
        // array with object
        (object, _) => object,
    }
}

/// See lodash [merge](https://lodash.com/docs/#merge)
///
/// Fn form: [merge()] | `_x` form: **not provided** — see [merge_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
///
/// let object = json!({
///   "a": [{ "b": 2 }, { "d": 4 }]
/// });
///
/// let other = json!({
///   "a": [{ "c": 3 }, { "e": 5 }]
/// });
///
/// assert_eq!(
///   merge!(object, other),
///   json!({ 'a': [{ 'b': 2, 'c': 3 }, { 'd': 4, 'e': 5 }] })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(merge!(), json!({}));
/// assert_eq!(merge!(json!({'a':1})), json!({'a':1}));
/// # assert_eq!(
/// #   merge!(json!({'a':1}), json!({'b':2}), ),
/// #   json!({'a':1, 'b':2})
/// # );
/// assert_eq!(
///   merge!(json!({'a':1}), json!({'b':2}), json!({'c':3})),
///   json!({'a': 1, 'b': 2, 'c': 3})
/// );
/// # assert_eq!(
/// #   merge!(json!({'a':1}), json!({'b':2}), json!({'c':3}), ),
/// #   json!({'a': 1, 'b': 2, 'c': 3})
/// # );
/// ```
#[macro_export]
macro_rules! merge {
    () => (
        $crate::lib::json!({})
    );
    ($a:expr $(,)*) => {
        $crate::merge($a, $crate::lib::json!({}))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::merge($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::merge!($crate::merge($a, $b), $($rest)*)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [merge!](crate::merge!) and read the returned `Value`.
///
/// Macro form: [merge_x!](crate::merge_x!)
pub fn merge_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [merge!](crate::merge!) and read the returned `Value`.
///
/// Fn form: [merge_x()]
#[macro_export]
macro_rules! merge_x {
    ($($t:tt)*) => {
        $crate::merge_x()
    };
}

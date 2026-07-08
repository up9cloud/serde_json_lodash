use crate::lib::{Map, Value, json};

fn item_to_string(v: Value) -> String {
    match v {
        Value::Null => "null".into(),
        other => crate::to_string_x(other),
    }
}

fn value_to_kv(v: Value) -> Option<(String, Option<Value>)> {
    match v {
        Value::String(s) => {
            let mut chars = s.chars();
            let k = chars.next()?.to_string();
            match chars.next() {
                Some(c) => Some((k, Some(Value::String(c.to_string())))),
                None => Some((k, None)),
            }
        }
        Value::Array(vec) => {
            let mut items = vec.into_iter();
            let k = item_to_string(items.next()?);
            Some((k, items.next()))
        }
        _ => None,
    }
}

fn append_array_to_object(array: Value, mut map: Map<String, Value>) -> Map<String, Value> {
    if let Some((k, v)) = value_to_kv(array) {
        if let Some(vv) = v {
            map.insert(k, vv);
        } else {
            map.remove(&k);
        }
    }
    map
}

fn arrays_to_object(vec: Vec<Value>) -> Value {
    let mut map = Map::new();
    for item in vec {
        map = append_array_to_object(item, map);
    }
    Value::Object(map)
}

/// Fn form of [from_pairs!](crate::from_pairs!); see it for the full docs
///
/// `_x` form: **not provided** — see [from_pairs_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::from_pairs;
/// # use serde_json::json;
/// assert_eq!(from_pairs(json!([['a', 1], ['b', 2]])), json!({ 'a': 1, 'b': 2 }));
/// ```
pub fn from_pairs(v: Value) -> Value {
    match v {
        Value::Null => json!({}),
        Value::Bool(_) => json!({}),
        Value::Number(_) => json!({}),
        Value::String(_) => json!({}),
        Value::Array(vec) => arrays_to_object(vec),
        Value::Object(_) => json!({}),
    }
}

/// See lodash [fromPairs](https://lodash.com/docs/#fromPairs)
///
/// Fn form: [from_pairs()] | `_x` form: **not provided** — see [from_pairs_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   from_pairs!(json!([['a', 1], ['b', 2]])),
///   json!({ 'a': 1, 'b': 2 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(from_pairs!(), json!({}));
/// assert_eq!(from_pairs!(json!(null)), json!({}));
/// assert_eq!(from_pairs!(json!(false)), json!({}));
/// assert_eq!(from_pairs!(json!(0)), json!({}));
/// assert_eq!(from_pairs!(json!("")), json!({}));
/// assert_eq!(from_pairs!(json!("ab")), json!({}));
/// assert_eq!(from_pairs!(json!("りしれ")), json!({}));
/// assert_eq!(from_pairs!(json!([])), json!({}));
/// assert_eq!(from_pairs!(json!(["a"])), json!({}));
/// assert_eq!(from_pairs!(json!(["ab"])), json!({"a":"b"}));
/// assert_eq!(from_pairs!(json!(["りしれ"])), json!({"り":"し"}));
/// assert_eq!(from_pairs!(json!(["ab","a"])), json!({}));
/// assert_eq!(from_pairs!(json!({"a":1})), json!({}));
/// assert_eq!(from_pairs!(json!([{}, [null,false]])), json!({"null":false}));
/// assert_eq!(from_pairs!(json!([[{},false]])), json!({"serde_json::map::Map<alloc::string::String, serde_json::value::Value>":false}));
/// assert_eq!(from_pairs!(json!(["aa",["a","b"],["a",["c"]]])), json!({"a":["c"]}));
/// ```
#[macro_export]
macro_rules! from_pairs {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::from_pairs($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::from_pairs($a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [from_pairs!](crate::from_pairs!) and read the returned
/// `Value`.
///
/// Macro form: [from_pairs_x!](crate::from_pairs_x!)
pub fn from_pairs_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [from_pairs!](crate::from_pairs!) and read the returned
/// `Value`.
///
/// Fn form: [from_pairs_x()]
#[macro_export]
macro_rules! from_pairs_x {
    ($($t:tt)*) => {
        $crate::from_pairs_x()
    };
}

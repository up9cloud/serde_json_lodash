use crate::lib::{json, Value, Map};

fn item_to_string(v: &Value) -> String {
    match v {
        Value::Null => "null".into(),
        Value::Bool(_)
        | Value::String(_)
        | Value::Number(_)
        | Value::Object(_)
        | Value::Array(_) => crate::to_string_x(v.clone()),
    }
}

fn value_to_kv(v: &Value) -> Option<(String, Option<Value>)> {
    match v {
        Value::Null => None,
        Value::Bool(_) => None,
        Value::Number(_) => None,
        Value::String(s) => {
            let mut chars = s.chars();
            if let Some(k) = chars.next() {
                let k = k.to_string();
                if let Some(v) = chars.next() {
                    return Some((k, Some(Value::String(v.to_string()))));
                }
                return Some((k, None));
            }
            None
        }
        Value::Array(vec) => {
            if let Some(k) = vec.get(0) {
                let k = item_to_string(k);
                if let Some(v) = vec.get(1) {
                    return Some((k, Some(v.clone())));
                }
                return Some((k, None));
            }
            None
        }
        Value::Object(_) => None,
    }
}

fn append_array_to_object(array: &Value, mut map: Map<String, Value>) -> Map<String, Value> {
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
    for item in vec.iter() {
        map = append_array_to_object(item, map);
    }
    Value::Object(map)
}

///
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

/// Description can be found in [lodash fromPairs](https://lodash.com/docs/#fromPairs)
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
/// More examples:
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
        json!({})
    };
    ($a:expr $(,)*) => {
        $crate::from_pairs($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::from_pairs($a)
    };
}

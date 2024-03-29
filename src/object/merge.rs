use crate::lib::{json, Value};

fn merge_2_array(object: Value, source: Value) -> Value {
    let mut new_v = vec![];
    let object_vec = object.as_array().unwrap();
    let object_vec_len = object_vec.len();
    if object_vec_len == 0 {
        return source;
    }
    let source_vec = source.as_array().unwrap();
    if source_vec.is_empty() {
        return object;
    }
    for (i, v) in source_vec.iter().enumerate() {
        if i > object_vec_len - 1 {
            new_v.push(v.clone());
            continue;
        }
        if v.is_object() && object_vec[i].is_object() {
            new_v.push(merge_2_object(object_vec[i].clone(), v.clone()));
            continue;
        }
        if v.is_array() && object_vec[i].is_array() {
            new_v.push(merge_2_array(object_vec[i].clone(), v.clone()));
            continue;
        }
        new_v.push(v.clone())
    }
    json!(new_v)
}
fn merge_2_object(mut object: Value, source: Value) -> Value {
    for (source_k, source_v) in source.as_object().unwrap().iter() {
        match object.get(source_k) {
            Some(object_v) => match object_v {
                Value::Null => {
                    if !source_v.is_null() {
                        *object.get_mut(source_k).unwrap() = source_v.clone();
                    }
                }
                Value::Bool(_) => {
                    *object.get_mut(source_k).unwrap() = source_v.clone();
                }
                Value::String(_) => {
                    *object.get_mut(source_k).unwrap() = source_v.clone();
                }
                Value::Number(_) => {
                    *object.get_mut(source_k).unwrap() = source_v.clone();
                }
                Value::Array(_) => {
                    if source_v.is_array() {
                        let new_v = merge_2_array(object_v.clone(), source_v.clone());
                        object
                            .as_object_mut()
                            .unwrap()
                            .insert(source_k.clone(), new_v);
                    } else {
                        *object.get_mut(source_k).unwrap() = source_v.clone();
                    }
                }
                Value::Object(_) => {
                    if source_v.is_object() {
                        let new_v = merge_2_object(object_v.clone(), source_v.clone());
                        object
                            .as_object_mut()
                            .unwrap()
                            .insert(source_k.clone(), new_v);
                    } else {
                        *object.get_mut(source_k).unwrap() = source_v.clone();
                    }
                }
            },
            None => {
                object
                    .as_object_mut()
                    .unwrap()
                    .insert(source_k.clone(), source_v.clone());
            }
        }
    }
    object
}
/// See lodash [merge](https://lodash.com/docs/#merge)
pub fn merge(object: Value, source: Value) -> Value {
    if object.is_object() && source.is_object() {
        return merge_2_object(object, source);
    }
    if object.is_array() && source.is_array() {
        return merge_2_array(object, source);
    }
    // TODO:
    // object with array
    // array with object
    object
}
/// Based on [merge()]
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
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(merge!(), json!({}));
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
        json!({})
    );
    ($a:expr $(,)*) => {
        $crate::merge($a, json!)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::merge($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::merge!($crate::merge($a, $b), $($rest)*)
    };
}

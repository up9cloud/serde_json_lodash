use crate::lib::{
    json,
    Value};
use crate::internal::value_undefined;
use crate::to_path_x;

/// See lodash [set](https://lodash.com/docs/#set)
pub fn set(mut object: Value, path: Value, value: Value) -> Value {
    let mut p_vec = to_path_x(path);
    if p_vec.is_empty() {
        return object;
    }
    let last = p_vec.pop();
    let i_max = p_vec.len() - 1;
    let mut cur = &mut object;
    for (ii, k) in p_vec.iter().enumerate() {
        match cur {
            Value::Array(vec) => match k.parse::<usize>() {
                Ok(i) => {
                    let len = vec.len();
                    if len <= i {
                        let v = cur.as_array_mut().unwrap();
                        for _ in len..(i+1) {
                            v.push(value_undefined());
                        }
                    }
                    cur = cur.get_mut(i).unwrap()
                },
                Err(_) => return object,
            },
            Value::Object(map) => {
                if !map.contains_key(k) {
                    let v = if ii + 1 <= i_max && p_vec[ii + 1].parse::<usize>().is_ok() {
                        json!([])
                    } else {
                        json!({})
                    };
                    cur.as_object_mut().unwrap().insert(k.to_owned(), v);
                }
                cur = cur.get_mut(k).unwrap()
            },
            _ => match k.parse::<usize>() {
                Ok(i) => {
                    *cur = json!([]);
                    for _ in 0..i+1 {
                        cur.as_array_mut().unwrap().push(value_undefined())
                    }
                    cur = &mut cur[i];
                },
                Err(_) => {
                    *cur = json!({k: {}});
                    cur = &mut cur[k];
                }
            },
        }
    }
    let k = last.unwrap();
    match cur {
        Value::Array(_) => match k.parse::<usize>() {
            Ok(n) => match cur.get_mut(n) {
                Some(v) => {
                    *v = value;
                },
                None => {
                    let inner = cur.as_array_mut().unwrap();
                    for _ in inner.len()..n {
                        inner.push(value_undefined())
                    }
                    inner.push(value);
                }
            },
            Err(_) => return object,
        },
        Value::Object(_) => match cur.get_mut(k.to_owned()) {
            Some(v) => {
                *v = value;
            },
            None => {
                cur.as_object_mut().unwrap().insert(k.to_owned(), value);
            }
        },
        _ => match k.parse::<usize>() {
            Ok(n) => {
                let mut base = json!([]);
                let inner = base.as_array_mut().unwrap();
                for _ in 0..n {
                    inner.push(value_undefined())
                }
                inner.push(value);
                *cur = base;
            },
            Err(_) => {
                *cur = json!({k: value});
            }
        },
    }
    object
}
/// Based on [set()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
///
/// let object = json!({
///   "a": [{ "b": { "c": 3 } }]
/// });
///
/// let object = set!(object, json!("a[0].b.c"), json!(4));
/// assert_eq!(
///   object["a"][0]["b"]["c"],
///   json!(4)
/// );
///
/// let object = set!(object, json!(["x", "0", "y", "z"]), json!(5));
/// assert_eq!(
///   object["x"][0]["y"]["z"],
///   json!(5)
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(set!(), json!(null));
/// assert_eq!(set!(json!(5)), json!(5));
/// assert_eq!(
///   set!(json!({}), json!("1[1].a"), json!(5)),
///   json!({"1":[null,{"a":5}]})
/// );
/// ```
#[macro_export]
macro_rules! set {
    () => (
        $crate::internal::value_undefined()
    );
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::set($a, $b, json!({}))
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::set($a, $b, $c)
    };
}

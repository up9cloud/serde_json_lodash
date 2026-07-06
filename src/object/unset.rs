use crate::lib::Value;
use crate::to_path_x;

/// See lodash [unset](https://lodash.com/docs/#unset)
///
/// Removes the property at `path`, returning the (possibly modified) object
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::unset;
/// # use serde_json::json;
/// assert_eq!(unset(json!({"a": 1, "b": 2}), json!("a")), json!({"b": 2}));
/// ```
pub fn unset(mut object: Value, path: Value) -> Value {
    let p_vec = to_path_x(path);
    if p_vec.is_empty() {
        return object;
    }
    let mut cur = &mut object;
    for k in &p_vec[..p_vec.len() - 1] {
        cur = match cur {
            Value::Object(o) => match o.get_mut(k) {
                Some(v) => v,
                None => return object,
            },
            Value::Array(vec) => match k.parse::<usize>() {
                Ok(i) if i < vec.len() => &mut vec[i],
                _ => return object,
            },
            _ => return object,
        };
    }
    let last = &p_vec[p_vec.len() - 1];
    match cur {
        Value::Object(o) => {
            o.remove(last);
        }
        Value::Array(vec) => {
            if let Ok(i) = last.parse::<usize>()
                && i < vec.len()
            {
                vec[i] = Value::Null;
            }
        }
        _ => {}
    }
    object
}

/// Based on [unset()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": [{ "b": { "c": 7 } }] });
/// assert_eq!(
///   unset!(object, json!("a[0].b.c")),
///   json!({ "a": [{ "b": {} }] })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(unset!(), json!(null));
/// assert_eq!(unset!(json!({"a": 1})), json!({"a": 1}));
/// assert_eq!(unset!(json!({"a": 1, "b": 2}), json!("a")), json!({"b": 2}));
/// ```
#[macro_export]
macro_rules! unset {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::unset($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::unset($a, $b)
    };
}

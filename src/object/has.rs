use crate::lib::Value;
use crate::to_path_x;

/// See lodash [has](https://lodash.com/docs/#has)
pub fn has(object: &Value, path: Value) -> bool {
    let p_vec = to_path_x(path);
    if p_vec.is_empty() {
        return false;
    }
    let mut cur = object;
    for k in p_vec.iter() {
        match cur {
            Value::Object(o) => match o.get(k) {
                Some(v) => cur = v,
                None => return false,
            },
            Value::Array(vec) => match k.parse::<usize>() {
                Ok(i) if i < vec.len() => cur = &vec[i],
                _ => return false,
            },
            _ => return false,
        }
    }
    true
}

/// Based on [has()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": { "b": 2 } });
/// assert_eq!(has!(&object, json!("a.b")), true);
/// assert_eq!(has!(&object, json!(["a", "b"])), true);
/// assert_eq!(has!(&object, json!("a.c")), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(has!(), false);
/// assert_eq!(has!(&json!({"a": 1})), false);
/// assert_eq!(has!(&json!({"a": [{"b": 3}]}), json!("a[0].b")), true);
/// ```
#[macro_export]
macro_rules! has {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::has($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::has($a, $b)
    };
}

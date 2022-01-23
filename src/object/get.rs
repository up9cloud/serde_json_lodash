use crate::lib::{
    json,
    Value};
use crate::to_path_x;

/// See lodash [get](https://lodash.com/docs/#get)
pub fn get(object: Value, path: Value, default: Value) -> Value {
    let p_vec = to_path_x(path);
    if p_vec.is_empty() {
        return default;
    }
    let mut cur: Value = object;
    for k in p_vec.iter() {
        cur = match cur {
            Value::String(s) => {
                match k.parse::<usize>() {
                    Ok(n) => match s.chars().nth(n) {
                        Some(s) => json!(s),
                        None => return default,
                    },
                    Err(_) => return default,
                }
            },
            Value::Array(_) => match k.parse::<usize>() {
                Ok(n) => match cur.get(n) {
                    Some(v) => v.clone(),
                    None => return default
                },
                Err(_) => return default,
            },
            Value::Object(_) => match cur.get(k) {
                Some(v) => v.clone(),
                None => return default
            },
            _ => return default
        }
    }
    return cur
}
/// Based on [get()]
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
/// assert_eq!(
///   get!(object.clone(), json!("a[0].b.c")),
///   json!(3)
/// );
/// assert_eq!(
///   get!(object.clone(), json!(["a", "0", "b", "c"])),
///   json!(3)
/// );
/// assert_eq!(
///   get!(object.clone(), json!("a.b.c"), json!("default")),
///   json!("default")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(
///   get!(json!("中文"), json!("1")),
///   json!("文")
/// );
/// assert_eq!(
///   get!(json!([1,"abcd"]), json!("1[2]")),
///   json!("c")
/// );
/// ```
#[macro_export]
macro_rules! get {
    () => (
        $crate::internal::value_undefined()
    );
    ($a:expr $(,)*) => {
        $crate::internal::value_undefined()
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::get($a, $b, $crate::internal::value_undefined())
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::get($a, $b, $c)
    };
}

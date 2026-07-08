use crate::lib::{Value, json};

use crate::to_path_x;

/// Fn form of [get!](crate::get!); see it for the full docs
///
/// `_x` form: **not provided** — see [get_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::get;
/// # use serde_json::json;
/// assert_eq!(get(json!({"a": {"b": 2}}), json!("a.b"), json!(null)), json!(2));
/// ```
pub fn get(object: Value, path: Value, default: Value) -> Value {
    let p_vec = to_path_x(path);
    if p_vec.is_empty() {
        return default;
    }
    get_in(&object, &p_vec).unwrap_or(default)
}

// Descends by reference and clones only the final value, so intermediate
// subtrees are never copied. Shared with [at()] / [update()], which resolve
// paths against a borrowed object.
pub(crate) fn get_in(object: &Value, path: &[String]) -> Option<Value> {
    let (k, rest) = match path.split_first() {
        Some(x) => x,
        None => return Some(object.clone()),
    };
    match object {
        Value::String(s) => {
            let c = s.chars().nth(k.parse::<usize>().ok()?)?;
            let v = json!(c);
            if rest.is_empty() {
                Some(v)
            } else {
                get_in(&v, rest)
            }
        }
        Value::Array(vec) => get_in(vec.get(k.parse::<usize>().ok()?)?, rest),
        Value::Object(map) => get_in(map.get(k)?, rest),
        _ => None,
    }
}

/// See lodash [get](https://lodash.com/docs/#get)
///
/// Fn form: [get()] | `_x` form: **not provided** — see [get_x()]
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
/// Additional cases:
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
    () => {
        $crate::internal::value_undefined()
    };
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

build_not_provided_x!(get, get_x);

use crate::lib::Value;

/// Type of a single `(key, predicate)` entry passed to [conforms_to()]
pub type Conform<'a> = (&'a str, fn(&Value) -> bool);

/// See lodash [conformsTo](https://lodash.com/docs/#conformsTo)
///
/// `source` maps property names to predicates, since predicates cannot be
/// stored inside a `serde_json::Value`
pub fn conforms_to(object: &Value, source: Vec<Conform>) -> bool {
    for (key, predicate) in source {
        match object.get(key) {
            Some(v) => {
                if !predicate(v) {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

/// Based on [conforms_to()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": 2 });
/// assert_eq!(
///   conforms_to!(&object, vec![("b", |n| n.as_i64().unwrap() > 1)]),
///   true
/// );
/// assert_eq!(
///   conforms_to!(&object, vec![("b", |n| n.as_i64().unwrap() > 2)]),
///   false
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(conforms_to!(), true);
/// assert_eq!(conforms_to!(&json!({})), true);
/// assert_eq!(conforms_to!(&json!({}), vec![("a", |_: &serde_json::Value| true)]), false);
/// ```
#[macro_export]
macro_rules! conforms_to {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::conforms_to($a, vec![])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::conforms_to($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::conforms_to($a, $b)
    };
}

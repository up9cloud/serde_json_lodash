use crate::lib::{json, Value};

/// See lodash [nth](https://lodash.com/docs/#nth)
pub fn nth(v: Value, n: isize) -> Value {
    match v {
        Value::Null => json!(null),
        Value::Bool(_) => json!(null),
        Value::Number(_) => json!(null),
        Value::String(s) => {
            if s.is_empty() {
                return json!(null);
            }
            let chars = s.chars();
            let nn = {
                if n < 0 {
                    let nn = (chars.count() as isize) + n;
                    if nn < 0 {
                        return json!(null);
                    }
                    nn
                } else {
                    n
                }
            };
            match s.chars().nth(nn as usize) {
                Some(v) => json!(v),
                None => json!(null),
            }
        }
        Value::Array(vec) => {
            if vec.is_empty() {
                return json!(null);
            }
            let nn = {
                if n < 0 {
                    let nn = (vec.len() as isize) + n;
                    if nn < 0 {
                        return json!(null);
                    }
                    nn
                } else {
                    n
                }
            };
            match vec.get(nn as usize) {
                Some(v) => v.clone(),
                None => json!(null),
            }
        }
        Value::Object(_) => json!(null),
    }
}

/// Based on [nth()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!(['a', 'b', 'c', 'd']);
/// assert_eq!(
///   nth!(array.clone(), 1),
///   json!('b')
/// );
/// assert_eq!(
///   nth!(array.clone(), -2),
///   json!('c')
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(nth!(), json!(null));
/// assert_eq!(nth!(json!(null)), json!(null));
/// assert_eq!(nth!(json!(false)), json!(null));
/// assert_eq!(nth!(json!(true)), json!(null));
/// assert_eq!(nth!(json!(0)), json!(null));
/// assert_eq!(nth!(json!("")), json!(null));
/// assert_eq!(nth!(json!("ab")), json!("a"));
/// assert_eq!(nth!(json!("冬至")), json!("冬"));
/// assert_eq!(nth!(json!("夏至"), -1), json!("至"));
/// assert_eq!(nth!(json!("春分"), -3), json!(null));
/// assert_eq!(nth!(json!("秋分"), 2), json!(null));
/// assert_eq!(nth!(json!([])), json!(null));
/// assert_eq!(nth!(json!({})), json!(null));
/// ```
#[macro_export]
macro_rules! nth {
    () => {
        json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::nth($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::nth($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::nth($a, $b)
    };
}

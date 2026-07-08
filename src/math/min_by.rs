use crate::lib::Value;

use crate::internal::value_to_option_number;

/// Fn form of [min_by!](crate::min_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [min_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::min_by;
/// # use serde_json::json;
/// assert_eq!(min_by(json!([3, 1, 2]), |v| v.clone()), json!(1));
/// ```
pub fn min_by(array: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => {
            let mut best: Option<(Value, f64)> = None;
            for v in vec {
                if let Some(n) = value_to_option_number(iteratee(&v)).and_then(|n| n.as_f64()) {
                    match &best {
                        Some((_, bn)) if n >= *bn => {}
                        _ => best = Some((v, n)),
                    }
                }
            }
            best.map(|(v, _)| v).unwrap_or(Value::Null)
        }
        _ => Value::Null,
    }
}

/// See lodash [minBy](https://lodash.com/docs/#minBy)
///
/// `iteratee` maps each element to the value used for comparison
///
/// Fn form: [min_by()] | `_x` form: **not provided** — see [min_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([{ "n": 1 }, { "n": 2 }]);
/// assert_eq!(
///   min_by!(objects, |o| o["n"].clone()),
///   json!({ "n": 1 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(min_by!(), json!(null));
/// assert_eq!(min_by!(json!([])), json!(null));
/// assert_eq!(min_by!(json!([3, 1, 2])), json!(1));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(min_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!({"a":0,"b":1}));
/// assert_eq!(min_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!({"a":3,"b":2}));
/// ```
#[macro_export]
macro_rules! min_by {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::min($a)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::min_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::min_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::min_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::min_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::min_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::min_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::min_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::min_by($a, $b)
    };
}

build_not_provided_x!(min_by, min_by_x);

use crate::lib::{Value, json};

/// Fn form of [take_right_while!](crate::take_right_while!); see it for the full docs
///
/// `_x` form: **not provided** — see [take_right_while_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::take_right_while;
/// # use serde_json::json;
/// assert_eq!(take_right_while(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() > 2), json!([3, 4]));
/// ```
pub fn take_right_while(array: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => {
            let mut n = 0;
            for v in vec.iter().rev() {
                if predicate(v) {
                    n += 1;
                } else {
                    break;
                }
            }
            Value::Array(vec[vec.len() - n..].to_vec())
        }
        _ => json!([]),
    }
}

/// See lodash [takeRightWhile](https://lodash.com/docs/#takeRightWhile)
///
/// Takes elements from the end while `predicate` returns `true`
///
/// Fn form: [take_right_while()] | `_x` form: **not provided** — see [take_right_while_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   take_right_while!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() > 2),
///   json!([3, 4])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(take_right_while!(), json!([]));
/// assert_eq!(take_right_while!(json!([1, 2, 3])), json!([]));
/// assert_eq!(take_right_while!(json!([1, 2, 3]), |_| true), json!([1, 2, 3]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(take_right_while!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([]));
/// assert_eq!(take_right_while!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["a", 2])), json!([]));
/// assert_eq!(take_right_while!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([{"a":2,"b":1},{"a":3,"b":2}]));
/// ```
#[macro_export]
macro_rules! take_right_while {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::take_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::take_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::take_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::take_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::take_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::take_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::take_right_while($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::take_right_while($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [take_right_while!](crate::take_right_while!) and read
/// the returned `Value`.
///
/// Macro form: [take_right_while_x!](crate::take_right_while_x!)
pub fn take_right_while_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [take_right_while!](crate::take_right_while!) and read
/// the returned `Value`.
///
/// Fn form: [take_right_while_x()]
#[macro_export]
macro_rules! take_right_while_x {
    ($($t:tt)*) => {
        $crate::take_right_while_x()
    };
}

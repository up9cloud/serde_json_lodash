use crate::lib::{Value, json};

/// Fn form of [drop_right_while!](crate::drop_right_while!); see it for the full docs
///
/// `_x` form: **not provided** — see [drop_right_while_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::drop_right_while;
/// # use serde_json::json;
/// assert_eq!(drop_right_while(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() > 2), json!([1, 2]));
/// ```
pub fn drop_right_while(array: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(mut vec) => {
            while let Some(last) = vec.last() {
                if predicate(last) {
                    vec.pop();
                } else {
                    break;
                }
            }
            Value::Array(vec)
        }
        _ => json!([]),
    }
}

/// See lodash [dropRightWhile](https://lodash.com/docs/#dropRightWhile)
///
/// Drops elements from the end while `predicate` returns `true`
///
/// Fn form: [drop_right_while()] | `_x` form: **not provided** — see [drop_right_while_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   drop_right_while!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() > 2),
///   json!([1, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(drop_right_while!(), json!([]));
/// assert_eq!(drop_right_while!(json!([1, 2, 3])), json!([1, 2, 3]));
/// assert_eq!(drop_right_while!(json!([1, 2, 3]), |_| true), json!([]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(drop_right_while!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([{"a":0,"b":1},{"a":2,"b":1},{"a":3,"b":2}]));
/// assert_eq!(drop_right_while!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["a", 2])), json!([{"a":0,"b":1},{"a":2,"b":1},{"a":3,"b":2}]));
/// assert_eq!(drop_right_while!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([{"a":0,"b":1}]));
/// ```
#[macro_export]
macro_rules! drop_right_while {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::drop_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::drop_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::drop_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::drop_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::drop_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::drop_right_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::drop_right_while($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::drop_right_while($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [drop_right_while!](crate::drop_right_while!) and read
/// the returned `Value`.
///
/// Macro form: [drop_right_while_x!](crate::drop_right_while_x!)
pub fn drop_right_while_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [drop_right_while!](crate::drop_right_while!) and read
/// the returned `Value`.
///
/// Fn form: [drop_right_while_x()]
#[macro_export]
macro_rules! drop_right_while_x {
    ($($t:tt)*) => {
        $crate::drop_right_while_x()
    };
}

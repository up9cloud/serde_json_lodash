use crate::lib::Value;

use crate::internal::value_shorthand;

/// Fn form of [iteratee!](crate::iteratee!); see it for the full docs
///
/// `_x` form: **not provided** — see [iteratee_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::iteratee;
/// # use serde_json::json;
/// assert_eq!(iteratee("a")(&json!({"a": 1})), json!(1));
/// ```
pub fn iteratee(spec: impl Into<Value>) -> impl Fn(&Value) -> Value {
    let spec = spec.into();
    move |v| value_shorthand(&spec, v)
}

/// See lodash [iteratee](https://lodash.com/docs/#iteratee)
///
/// Returns a closure implementing the lodash iteratee shorthand for `spec`:
/// an object is a partial deep match (`_.matches`), a `[path, value]` pair is
/// `_.matchesProperty`, `null` is `_.identity` and anything else is a
/// `_.property` path.
///
/// The predicate-style macros (`filter!`, `find!`, `every!`, …) and
/// iteratee-style macros (`map!`, `sort_by!`, …) also accept an inline
/// `json!(…)` or string literal directly, which goes through this same
/// dispatch.
///
/// Fn form: [iteratee()] | `_x` form: **not provided** — see [iteratee_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "barney", "age": 36, "active": true },
///   { "user": "fred",   "age": 40, "active": false }
/// ]);
///
/// // The `_.matches` iteratee shorthand.
/// assert_eq!(
///   filter!(users.clone(), json!({ "user": "barney", "active": true })),
///   json!([{ "user": "barney", "age": 36, "active": true }])
/// );
///
/// // The `_.matchesProperty` iteratee shorthand.
/// assert_eq!(
///   filter!(users.clone(), json!(["user", "fred"])),
///   json!([{ "user": "fred", "age": 40, "active": false }])
/// );
///
/// // The `_.property` iteratee shorthand.
/// assert_eq!(map!(users, iteratee!("user")), json!(["barney", "fred"]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(iteratee!(json!(null))(&json!(1)), json!(1)); // null is identity
/// assert_eq!(iteratee!(json!({"a": 1}))(&json!({"a": 1, "b": 2})), json!(true));
/// assert_eq!(iteratee!(json!(["a", 1]))(&json!({"a": 2})), json!(false));
/// ```
#[macro_export]
macro_rules! iteratee {
    () => {
        $crate::iteratee($crate::lib::json!(null))
    };
    ($a:expr $(,)*) => {
        $crate::iteratee($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::iteratee($a)
    };
}

/// **Not provided.** The result is a function, which has no primitive form;
/// use [iteratee!](crate::iteratee!) and call the returned closure.
///
/// Macro form: [iteratee_x!](crate::iteratee_x!)
pub fn iteratee_x() {
    todo!()
}

/// **Not provided.** The result is a function, which has no primitive form;
/// use [iteratee!](crate::iteratee!) and call the returned closure.
///
/// Fn form: [iteratee_x()]
#[macro_export]
macro_rules! iteratee_x {
    ($($t:tt)*) => {
        $crate::iteratee_x()
    };
}

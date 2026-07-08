use crate::lib::Value;

use crate::internal::base_is_match;

/// Fn form of [matches!](crate::matches!); see it for the full docs
///
/// `_x` form: **not provided** — see [matches_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::matches;
/// # use serde_json::json;
/// assert_eq!(matches(json!({"a": 4}))(&json!({"a": 4, "b": 5})), true);
/// ```
pub fn matches(source: impl Into<Value>) -> impl Fn(&Value) -> bool {
    let source = source.into();
    move |v| base_is_match(v, &source)
}

/// See lodash [matches](https://lodash.com/docs/#matches)
///
/// Returns a predicate closure doing a partial deep comparison against
/// `source` (see [is_match!](crate::is_match!)).
///
/// *Note:* this exported `matches!` shadows [`std::matches!`] under
/// `#[macro_use]`; call the std one as `std::matches!` if you need both.
///
/// Fn form: [matches()] | `_x` form: **not provided** — see [matches_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([
///   { "a": 1, "b": 2, "c": 3 },
///   { "a": 4, "b": 5, "c": 6 }
/// ]);
/// assert_eq!(
///   filter!(objects, matches!(json!({ "a": 4, "c": 6 }))),
///   json!([{ "a": 4, "b": 5, "c": 6 }])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(matches!(json!({}))(&json!({"a": 1})), true);
/// assert_eq!(matches!()(&json!({"a": 1})), true);
/// ```
#[macro_export]
macro_rules! matches {
    () => {
        $crate::matches($crate::lib::json!({}))
    };
    ($a:expr $(,)*) => {
        $crate::matches($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::matches($a)
    };
}

/// **Not provided.** The result is a predicate function, which has no
/// primitive form; use [matches!](crate::matches!) and call the returned
/// closure.
///
/// Macro form: [matches_x!](crate::matches_x!)
pub fn matches_x() {
    todo!()
}

/// **Not provided.** The result is a predicate function, which has no
/// primitive form; use [matches!](crate::matches!) and call the returned
/// closure.
///
/// Fn form: [matches_x()]
#[macro_export]
macro_rules! matches_x {
    ($($t:tt)*) => {
        $crate::matches_x()
    };
}

use crate::lib::{json, Value};
use crate::internal;

/// `x_`/`_x` helper for [start_case()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_start_case_x;
/// # use serde_json::json;
/// assert_eq!(x_start_case_x("--foo-bar--"), "Foo Bar".to_owned());
/// ```
pub fn x_start_case_x(s: &str) -> String {
    internal::compound_words(s)
        .iter()
        .map(|w| internal::upper_first_word(w))
        .collect::<Vec<_>>()
        .join(" ")
}
/// `x_` helper for [start_case()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_start_case;
/// # use serde_json::json;
/// assert_eq!(x_start_case("--foo-bar--"), json!("Foo Bar"));
/// ```
pub fn x_start_case(s: &str) -> Value {
    json!(x_start_case_x(s))
}
/// `_x` helper for [start_case()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::start_case_x;
/// # use serde_json::json;
/// assert_eq!(start_case_x(json!("--foo-bar--")), "Foo Bar".to_owned());
/// ```
pub fn start_case_x(v: Value) -> String {
    x_start_case_x(&crate::to_string_x(v))
}
/// See lodash [startCase](https://lodash.com/docs/#startCase)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::start_case;
/// # use serde_json::json;
/// assert_eq!(start_case(json!("--foo-bar--")), json!("Foo Bar"));
/// ```
pub fn start_case(v: Value) -> Value {
    json!(start_case_x(v))
}

/// Based on [start_case()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   start_case!(json!("--foo-bar--")),
///   json!("Foo Bar")
/// );
/// assert_eq!(
///   start_case!(json!("fooBar")),
///   json!("Foo Bar")
/// );
/// assert_eq!(
///   start_case!(json!("__FOO_BAR__")),
///   json!("FOO BAR")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(start_case!(), json!(""));
/// assert_eq!(start_case!(json!(null)), json!(""));
/// ```
#[macro_export]
macro_rules! start_case {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::start_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::start_case($a)
    };
}

/// Based on [x_start_case_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_start_case_x!("--foo-bar--"), "Foo Bar".to_owned());
/// ```
macro_rules! x_start_case_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_start_case_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_start_case_x($a)
    };
}
/// Based on [x_start_case()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(x_start_case!("--foo-bar--"), json!("Foo Bar"));
/// ```
macro_rules! x_start_case {
    () => {
        $crate::lib::json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_start_case($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_start_case($a)
    };
}
/// Based on [start_case_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(start_case_x!(json!("--foo-bar--")), "Foo Bar".to_owned());
/// ```
macro_rules! start_case_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::start_case_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::start_case_x($a)
    };
}

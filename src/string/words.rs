use crate::lib::{Value, json};

use crate::internal;

// internal worker for [words()].
fn x_words(s: &str) -> Value {
    json!(internal::words_vec(s))
}

/// Fn form of [words!](crate::words!); see it for the full docs
///
/// `_x` form: **not provided** — see [words_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::words;
/// # use serde_json::json;
/// assert_eq!(words(json!("fred, barney, & pebbles")), json!(["fred", "barney", "pebbles"]));
/// ```
pub fn words<A: Into<Value>>(v: A) -> Value {
    let v = v.into();
    x_words(&crate::to_string_x(v))
}

/// See lodash [words](https://lodash.com/docs/#words)
///
/// *Note:* the `[pattern]` parameter is not supported, only the default
/// lodash word splitting logic is implemented
///
/// Fn form: [words()] | `_x` form: **not provided** — see [words_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   words!(json!("fred, barney, & pebbles")),
///   json!(["fred", "barney", "pebbles"])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(words!(), json!([]));
/// assert_eq!(words!(json!(null)), json!([]));
/// assert_eq!(words!(json!("fred12barney")), json!(["fred", "12", "barney"]));
/// assert_eq!(words!(json!("FOOBar")), json!(["FOO", "Bar"]));
/// assert_eq!(words!(json!("don't, oh my")), json!(["don't", "oh", "my"]));
/// ```
#[macro_export]
macro_rules! words {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::words($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::words($a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [words!](crate::words!) and read the returned `Value`.
///
/// Macro form: [words_x!](crate::words_x!)
pub fn words_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [words!](crate::words!) and read the returned `Value`.
///
/// Fn form: [words_x()]
#[macro_export]
macro_rules! words_x {
    ($($t:tt)*) => {
        $crate::words_x()
    };
}

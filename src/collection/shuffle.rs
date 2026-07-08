use crate::lib::Value;

use crate::internal::rand_f64;

use crate::collection::collect::collection_values;

pub(crate) fn shuffle_vec(mut vec: Vec<Value>) -> Vec<Value> {
    // Fisher-Yates
    let len = vec.len();
    for i in (1..len).rev() {
        let j = (rand_f64() * (i + 1) as f64) as usize;
        vec.swap(i, j.min(i));
    }
    vec
}

/// Fn form of [shuffle!](crate::shuffle!); see it for the full docs
///
/// `_x` form: **not provided** — see [shuffle_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::shuffle;
/// # use serde_json::json;
/// assert_eq!(shuffle(json!([])), json!([]));
/// ```
pub fn shuffle(collection: Value) -> Value {
    Value::Array(shuffle_vec(collection_values(&collection)))
}

/// See lodash [shuffle](https://lodash.com/docs/#shuffle)
///
/// Fn form: [shuffle()] | `_x` form: **not provided** — see [shuffle_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let s = shuffle!(json!([1, 2, 3, 4]));
/// assert_eq!(s.as_array().unwrap().len(), 4);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(shuffle!(), json!([]));
/// assert_eq!(shuffle!(json!([])), json!([]));
/// assert_eq!(shuffle!(json!([9])), json!([9]));
/// ```
#[macro_export]
macro_rules! shuffle {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::shuffle($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::shuffle($a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [shuffle!](crate::shuffle!) and read the returned
/// `Value`.
///
/// Macro form: [shuffle_x!](crate::shuffle_x!)
pub fn shuffle_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [shuffle!](crate::shuffle!) and read the returned
/// `Value`.
///
/// Fn form: [shuffle_x()]
#[macro_export]
macro_rules! shuffle_x {
    ($($t:tt)*) => {
        $crate::shuffle_x()
    };
}

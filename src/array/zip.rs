use crate::lib::Value;

/// Zips a list of arrays together (the variadic form backing [zip()]).
///
/// This is an implementation detail exposed only so the `zip!` macro can
/// expand to it from other crates; prefer [zip()] or the `zip!` macro.
#[doc(hidden)]
pub fn zip_arrays(arrays: Vec<Value>) -> Value {
    let groups: Vec<Vec<Value>> = arrays
        .into_iter()
        .filter_map(|a| match a {
            Value::Array(v) => Some(v),
            _ => None,
        })
        .collect();
    let max_len = groups.iter().map(|g| g.len()).max().unwrap_or(0);
    let mut out = vec![];
    for i in 0..max_len {
        let tuple: Vec<Value> = groups
            .iter()
            .map(|g| g.get(i).cloned().unwrap_or(Value::Null))
            .collect();
        out.push(Value::Array(tuple));
    }
    Value::Array(out)
}

/// See lodash [zip](https://lodash.com/docs/#zip)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::zip;
/// # use serde_json::json;
/// assert_eq!(zip(json!(["a", "b"]), json!([1, 2])), json!([["a", 1], ["b", 2]]));
/// ```
pub fn zip(array: Value, other: Value) -> Value {
    zip_arrays(vec![array, other])
}

/// Based on [zip()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   zip!(json!(["a", "b"]), json!([1, 2]), json!([true, false])),
///   json!([["a", 1, true], ["b", 2, false]])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(zip!(), json!([]));
/// assert_eq!(zip!(json!(["a", "b"]), json!([1, 2])), json!([["a", 1], ["b", 2]]));
/// assert_eq!(zip!(json!(["a"]), json!([1, 2])), json!([["a", 1], [null, 2]]));
/// ```
#[macro_export]
macro_rules! zip {
    () => {
        $crate::lib::json!([])
    };
    ($($a:expr),+ $(,)*) => {
        $crate::zip_arrays(vec![$($a),+])
    };
}

/// `_x` helper for [zip()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [zip()] and read the returned `Value`.
pub fn zip_x() {
    todo!()
}
/// Based on [zip_x()]
#[macro_export]
macro_rules! zip_x {
    ($($t:tt)*) => {
        $crate::zip_x()
    };
}

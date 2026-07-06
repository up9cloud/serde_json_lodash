use crate::lib::Value;

fn count_eq(all: &[Value], v: &Value) -> usize {
    all.iter().filter(|x| *x == v).count()
}

/// See lodash [xor](https://lodash.com/docs/#xor)
///
/// Returns the symmetric difference: values present in exactly one of the
/// two arrays
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::xor;
/// # use serde_json::json;
/// assert_eq!(xor(json!([2, 1]), json!([2, 3])), json!([1, 3]));
/// ```
pub fn xor(array: Value, other: Value) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => vec![],
    };
    let b = match other {
        Value::Array(v) => v,
        _ => vec![],
    };
    let mut out = vec![];
    for v in a.iter() {
        if count_eq(&b, v) == 0 && !out.contains(v) {
            out.push(v.clone());
        }
    }
    for v in b.iter() {
        if count_eq(&a, v) == 0 && !out.contains(v) {
            out.push(v.clone());
        }
    }
    Value::Array(out)
}

/// Based on [xor()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(xor!(json!([2, 1]), json!([2, 3])), json!([1, 3]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(xor!(), json!([]));
/// assert_eq!(xor!(json!([1, 2])), json!([1, 2]));
/// assert_eq!(xor!(json!([1, 2]), json!([1, 2])), json!([]));
/// ```
#[macro_export]
macro_rules! xor {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $($b:expr),+ $(,)*) => {{
        let mut acc = $crate::uniq($a);
        $(
            acc = $crate::xor(acc, $b);
        )+
        acc
    }};
}

/// `_x` helper for [xor()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [xor()] and read the returned `Value`.
pub fn xor_x() {
    todo!()
}
/// Based on [xor_x()]
#[macro_export]
macro_rules! xor_x {
    ($($t:tt)*) => {
        $crate::xor_x()
    };
}

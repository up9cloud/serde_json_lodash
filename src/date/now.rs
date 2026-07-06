use crate::lib::{json, Value};

/// See lodash [now](https://lodash.com/docs/#now)
///
/// Returns the number of milliseconds that have elapsed since the Unix epoch
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::now;
/// # use serde_json::json;
/// let t = now();
/// assert!(t.as_u64().unwrap() > 0);
/// ```
pub fn now() -> Value {
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    json!(ms)
}

/// Based on [now()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let t = now!();
/// assert!(t.as_u64().unwrap() > 0);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// // milliseconds since the Unix epoch, well past the year 2020
/// assert!(now!().as_u64().unwrap() > 1_500_000_000_000);
/// ```
#[macro_export]
macro_rules! now {
    () => {
        $crate::now()
    };
    ($($rest:tt)*) => {
        $crate::now()
    };
}

/// `_x` helper for [now()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [now()] and read the returned `Value`.
pub fn now_x() {
    todo!()
}

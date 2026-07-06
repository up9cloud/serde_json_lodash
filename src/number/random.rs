use crate::lib::{Value, json};
use crate::internal::{f64_to_number, rand_f64, value_to_option_number};

/// See lodash [random](https://lodash.com/docs/#random)
///
/// Returns a random number between `lower` and `upper` (inclusive). If
/// `floating` is `true`, or either bound is not an integer, a floating point
/// number is returned
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::random;
/// # use serde_json::json;
/// let n = random(json!(0), json!(5), false);
/// assert!(n.as_i64().unwrap() >= 0 && n.as_i64().unwrap() <= 5);
/// ```
pub fn random(lower: Value, upper: Value, floating: bool) -> Value {
    let lo = value_to_option_number(lower)
        .and_then(|n| n.as_f64())
        .unwrap_or(0.0);
    let up = value_to_option_number(upper)
        .and_then(|n| n.as_f64())
        .unwrap_or(1.0);
    let (lo, up) = if lo <= up { (lo, up) } else { (up, lo) };
    let is_int = lo.fract() == 0.0 && up.fract() == 0.0;
    if floating || !is_int {
        let result = lo + rand_f64() * (up - lo);
        json!(result)
    } else {
        // inclusive of both bounds
        let span = (up - lo + 1.0).max(1.0);
        let result = lo + (rand_f64() * span).floor();
        match f64_to_number(result.min(up)) {
            Some(n) => Value::Number(n),
            None => json!(lo),
        }
    }
}

/// Based on [random()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let n = random!(json!(0), json!(5));
/// assert!(n.as_i64().unwrap() >= 0 && n.as_i64().unwrap() <= 5);
/// let f = random!(json!(1.2), json!(5.2));
/// assert!(f.as_f64().unwrap() >= 1.2 && f.as_f64().unwrap() <= 5.2);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// let n = random!();
/// assert!(n.as_i64().unwrap() == 0 || n.as_i64().unwrap() == 1);
/// // single argument is treated as the upper bound (lower defaults to 0)
/// let n = random!(json!(5));
/// assert!(n.as_i64().unwrap() >= 0 && n.as_i64().unwrap() <= 5);
/// // floating flag forces a float result
/// let f = random!(json!(0), json!(5), true);
/// assert!(f.as_f64().unwrap() >= 0.0 && f.as_f64().unwrap() <= 5.0);
/// ```
#[macro_export]
macro_rules! random {
    () => {
        $crate::random($crate::lib::json!(0), $crate::lib::json!(1), false)
    };
    ($a:expr $(,)*) => {
        $crate::random($crate::lib::json!(0), $a, false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::random($a, $b, false)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::random($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::random($a, $b, $c)
    };
}

/// `_x` helper for [random()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [random()] and read the returned `Value`.
pub fn random_x() {
    todo!()
}

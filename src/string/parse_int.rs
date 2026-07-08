use crate::lib::{Value, json};

use crate::internal::value_nan;

// internal worker for [parse_int()].
fn x_parse_int(s: &str, radix: u32) -> Value {
    let s = s.trim();
    let (negative, s) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.strip_prefix('+').unwrap_or(s)),
    };
    let (radix, s) = if radix == 0 {
        match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            Some(rest) => (16, rest),
            None => (10, s),
        }
    } else if radix == 16 {
        (
            16,
            s.strip_prefix("0x")
                .or_else(|| s.strip_prefix("0X"))
                .unwrap_or(s),
        )
    } else {
        (radix, s)
    };
    if !(2..=36).contains(&radix) {
        return value_nan();
    }
    let digits: String = s.chars().take_while(|c| c.is_digit(radix)).collect();
    match i64::from_str_radix(&digits, radix) {
        Ok(n) => {
            let n = if negative { -n } else { n };
            json!(n)
        }
        Err(_) => value_nan(),
    }
}

/// Fn form of [parse_int!](crate::parse_int!); see it for the full docs
///
/// `_x` form: **not provided** — see [parse_int_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::parse_int;
/// # use serde_json::json;
/// assert_eq!(parse_int(json!("10"), 2), json!(2));
/// ```
pub fn parse_int<A: Into<Value>>(v: A, radix: u32) -> Value {
    let v = v.into();
    x_parse_int(&crate::to_string_x(v), radix)
}

/// See lodash [parseInt](https://lodash.com/docs/#parseInt)
///
/// `radix = 0` means auto detection (`0x` prefixed strings are parsed as
/// hexadecimal, everything else as decimal), same as the lodash default.
/// Unparsable input returns `Value::Null` (there is no `NaN` in serde_json)
///
/// Fn form: [parse_int()] | `_x` form: **not provided** — see [parse_int_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   parse_int!(json!("08")),
///   json!(8)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(parse_int!(), json!(null));
/// assert_eq!(parse_int!(json!("0x1F")), json!(31));
/// assert_eq!(parse_int!(json!("42px")), json!(42));
/// assert_eq!(parse_int!(json!("-10")), json!(-10));
/// assert_eq!(parse_int!(json!("10"), 2), json!(2));
/// assert_eq!(parse_int!(json!("abc")), json!(null));
/// ```
#[macro_export]
macro_rules! parse_int {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::parse_int($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::parse_int($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::parse_int($a, $b)
    };
}

build_not_provided_x!(parse_int, parse_int_x);

use crate::lib::{json, Value};

static DEBURR_MAP: &[(&str, &str)] = &[
    ("ÀÁÂÃÄÅĀĂĄ", "A"),
    ("àáâãäåāăą", "a"),
    ("ÇĆĈĊČ", "C"),
    ("çćĉċč", "c"),
    ("ÐĎĐ", "D"),
    ("ðďđ", "d"),
    ("ÈÉÊËĒĔĖĘĚ", "E"),
    ("èéêëēĕėęě", "e"),
    ("ĜĞĠĢ", "G"),
    ("ĝğġģ", "g"),
    ("ĤĦ", "H"),
    ("ĥħ", "h"),
    ("ÌÍÎÏĨĪĬĮİ", "I"),
    ("ìíîïĩīĭįı", "i"),
    ("Ĵ", "J"),
    ("ĵ", "j"),
    ("Ķ", "K"),
    ("ķĸ", "k"),
    ("ĹĻĽĿŁ", "L"),
    ("ĺļľŀł", "l"),
    ("ÑŃŅŇŊ", "N"),
    ("ñńņňŋ", "n"),
    ("ÒÓÔÕÖØŌŎŐ", "O"),
    ("òóôõöøōŏő", "o"),
    ("ŔŖŘ", "R"),
    ("ŕŗř", "r"),
    ("ŚŜŞŠ", "S"),
    ("śŝşšſ", "s"),
    ("ŢŤŦ", "T"),
    ("ţťŧ", "t"),
    ("ÙÚÛÜŨŪŬŮŰŲ", "U"),
    ("ùúûüũūŭůűų", "u"),
    ("Ŵ", "W"),
    ("ŵ", "w"),
    ("ÝŶ", "Y"),
    ("ýÿŷ", "y"),
    ("ŹŻŽ", "Z"),
    ("źżž", "z"),
    ("Æ", "Ae"),
    ("æ", "ae"),
    ("Þ", "Th"),
    ("þ", "th"),
    ("ß", "ss"),
    ("Ĳ", "IJ"),
    ("ĳ", "ij"),
    ("Œ", "Oe"),
    ("œ", "oe"),
    ("ŉ", "'n"),
];

/// `x_`/`_x` helper for [deburr()]: takes a primitive argument and returns a primitive value.
pub fn x_deburr_x(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    'outer: for c in s.chars() {
        // combining diacritical marks are stripped
        if ('\u{300}'..='\u{36f}').contains(&c) {
            continue;
        }
        if !c.is_ascii() {
            for (from, to) in DEBURR_MAP {
                if from.contains(c) {
                    out.push_str(to);
                    continue 'outer;
                }
            }
        }
        out.push(c);
    }
    out
}
/// `x_` helper for [deburr()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
pub fn x_deburr(s: &str) -> Value {
    json!(x_deburr_x(s))
}
/// `_x` helper for [deburr()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn deburr_x(v: Value) -> String {
    x_deburr_x(&crate::to_string_x(v))
}
/// See lodash [deburr](https://lodash.com/docs/#deburr)
pub fn deburr(v: Value) -> Value {
    json!(deburr_x(v))
}

/// Based on [deburr()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   deburr!(json!("déjà vu")),
///   json!("deja vu")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(deburr!(), json!(""));
/// assert_eq!(deburr!(json!(null)), json!(""));
/// assert_eq!(deburr!(json!("Æthelred")), json!("Aethelred"));
/// assert_eq!(x_deburr!("crème brûlée"), json!("creme brulee"));
/// assert_eq!(x_deburr_x!("ss"), "ss".to_owned());
/// assert_eq!(deburr_x!(json!(123)), "123".to_owned());
/// ```
#[macro_export]
macro_rules! deburr {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::deburr($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::deburr($a)
    };
}
/// Based on [x_deburr()]
#[macro_export]
macro_rules! x_deburr {
    () => {
        json!("")
    };
    ($a:expr $(,)*) => {
        $crate::x_deburr($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_deburr($a)
    };
}
/// Based on [deburr_x()]
#[macro_export]
macro_rules! deburr_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::deburr_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::deburr_x($a)
    };
}
/// Based on [x_deburr_x()]
#[macro_export]
macro_rules! x_deburr_x {
    () => {
        "".to_owned()
    };
    ($a:expr $(,)*) => {
        $crate::x_deburr_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::x_deburr_x($a)
    };
}

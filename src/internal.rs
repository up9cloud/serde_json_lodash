use crate::lib::{Number, Value, json};

pub(crate) fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
// `value_undefined` and `value_nan` stay `pub` because exported macros
// (e.g. `ceil!`, `clamp!`) expand to `$crate::internal::...` in other
// crates; the rest of this module is crate-internal.
pub fn value_undefined() -> Value {
    json!(null)
}
pub fn value_nan() -> Value {
    json!(f64::NAN)
}
pub(crate) fn number_nan() -> Number {
    Number::from(0)
}
pub(crate) fn string_to_option_number(s: String) -> Option<Number> {
    if s.is_empty() {
        Some(0.into())
    } else if let Ok(n) = s.parse::<usize>() {
        Some(n.into())
    } else if let Ok(n) = s.parse::<isize>() {
        Some(n.into())
    } else if let Ok(n) = s.parse::<f64>() {
        Number::from_f64(n)
    } else {
        None
    }
}
pub(crate) fn vec_value_to_option_number(vec: Vec<Value>) -> Option<Number> {
    match vec.len() {
        0 => Some(value_null_to_number()),
        1 => value_to_option_number(vec[0].clone()),
        _ => None,
    }
}
pub(crate) fn value_null_to_number() -> Number {
    Number::from(0)
}
pub(crate) fn bool_to_number(b: bool) -> Number {
    if b { Number::from(1) } else { Number::from(0) }
}
// Deduplicate values by an iteratee-derived key, using value (deep) equality.
// lodash uses SameValueZero (reference identity for objects); for owned JSON
// values deep equality is the sensible equivalent. `Value: Hash` is
// consistent with its `Eq`, so a hash set keeps this O(n).
pub(crate) fn uniq_by_key(vec: Vec<Value>, key: impl Fn(&Value) -> Value) -> Vec<Value> {
    let mut seen: std::collections::HashSet<Value> = std::collections::HashSet::new();
    let mut out = vec![];
    for v in vec {
        if seen.insert(key(&v)) {
            out.push(v);
        }
    }
    out
}

// A random f64 in [0, 1), seeded from the OS via `RandomState` (whose hasher
// keys are randomized per process) mixed with a monotonically increasing
// counter, so we do not need an external rng crate
pub(crate) fn rand_f64() -> f64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(COUNTER.fetch_add(1, Ordering::Relaxed));
    if let Ok(d) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        hasher.write_u128(d.as_nanos());
    }
    // top 53 bits give a uniformly distributed f64 in [0, 1)
    (hasher.finish() >> 11) as f64 / (1u64 << 53) as f64
}

// Build a Number from an f64, preferring an integer representation (so the
// result serializes as `20` instead of `20.0`), same as JS number output
pub(crate) fn f64_to_number(f: f64) -> Option<Number> {
    if f.is_finite() && f.fract() == 0.0 {
        if f >= 0.0 && f <= u64::MAX as f64 {
            return Some(Number::from(f as u64));
        }
        if f >= i64::MIN as f64 && f < 0.0 {
            return Some(Number::from(f as i64));
        }
    }
    Number::from_f64(f)
}

// JS-ish relational comparison: strings compare lexicographically when both
// sides are strings, otherwise both sides are coerced to numbers
pub(crate) fn compare_values(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    if let (Value::String(sa), Value::String(sb)) = (a, b) {
        return Some(sa.cmp(sb));
    }
    let na = value_to_option_number(a.clone())?.as_f64()?;
    let nb = value_to_option_number(b.clone())?.as_f64()?;
    na.partial_cmp(&nb)
}

// lodash partial deep comparison: every key of `source` must match in
// `object` (recursively); arrays match when every source item matches some
// item of the object array
pub(crate) fn base_is_match(object: &Value, source: &Value) -> bool {
    match source {
        Value::Object(so) => match object {
            Value::Object(oo) => so
                .iter()
                .all(|(k, sv)| oo.get(k).is_some_and(|ov| base_is_match(ov, sv))),
            _ => so.is_empty(),
        },
        Value::Array(sa) => match object {
            Value::Array(oa) => sa
                .iter()
                .all(|sv| oa.iter().any(|ov| base_is_match(ov, sv))),
            _ => sa.is_empty(),
        },
        _ => object == source,
    }
}

#[derive(PartialEq, Clone, Copy)]
enum CharKind {
    Upper,
    Lower,
    Digit,
    Other,
}
fn char_kind(c: char) -> CharKind {
    if c.is_ascii_digit() {
        CharKind::Digit
    } else if c.is_alphabetic() {
        if c.is_uppercase() {
            CharKind::Upper
        } else {
            CharKind::Lower
        }
    } else {
        CharKind::Other
    }
}
// Splits a string into words, the same way lodash `_.words()` does:
// on non-alphanumeric separators, lower->Upper boundaries, letter<->digit
// boundaries and ALLCAPSWord boundaries; keeps contractions like "don't".
pub(crate) fn words_vec(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut words: Vec<String> = vec![];
    let mut current = String::new();
    let mut prev: Option<CharKind> = None;
    for (i, &c) in chars.iter().enumerate() {
        let kind = char_kind(c);
        match kind {
            CharKind::Other => {
                if (c == '\'' || c == '\u{2019}')
                    && prev == Some(CharKind::Lower)
                    && i + 1 < chars.len()
                    && char_kind(chars[i + 1]) == CharKind::Lower
                {
                    current.push(c);
                    continue;
                }
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
                prev = None;
            }
            _ => {
                let split = match prev {
                    None => false,
                    Some(p) => {
                        (p == CharKind::Digit) != (kind == CharKind::Digit)
                            || (p == CharKind::Lower && kind == CharKind::Upper)
                            || (p == CharKind::Upper
                                && kind == CharKind::Upper
                                && i + 1 < chars.len()
                                && char_kind(chars[i + 1]) == CharKind::Lower)
                    }
                };
                if split && !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
                current.push(c);
                prev = Some(kind);
            }
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}
// lodash compound case functions (camelCase, kebabCase, ...) all work on
// `words(deburr(string).replace(/['’]/g, ''))`
pub(crate) fn compound_words(s: &str) -> Vec<String> {
    let deburred = crate::deburr_x(s);
    let no_apos: String = deburred
        .chars()
        .filter(|c| *c != '\'' && *c != '\u{2019}')
        .collect();
    words_vec(&no_apos)
}
// Upper first char, lower the rest, e.q. "fOO" -> "Foo"
pub(crate) fn capitalize_word(w: &str) -> String {
    let mut cs = w.chars();
    match cs.next() {
        Some(c) => {
            let mut s = c.to_uppercase().to_string();
            s.push_str(&cs.as_str().to_lowercase());
            s
        }
        None => String::new(),
    }
}
// Upper first char, keep the rest, e.q. "fOO" -> "FOO"
pub(crate) fn upper_first_word(w: &str) -> String {
    let mut cs = w.chars();
    match cs.next() {
        Some(c) => {
            let mut s = c.to_uppercase().to_string();
            s.push_str(cs.as_str());
            s
        }
        None => String::new(),
    }
}
pub(crate) fn value_to_option_number(value: Value) -> Option<Number> {
    match value {
        Value::Null => Some(value_null_to_number()),
        Value::Bool(b) => Some(bool_to_number(b)),
        Value::Number(n) => Some(n),
        Value::String(s) => string_to_option_number(s),
        Value::Array(vec) => vec_value_to_option_number(vec),
        Value::Object(_) => None,
    }
}

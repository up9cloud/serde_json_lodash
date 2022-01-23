use crate::lib::{json, Value, Number};

pub const MAX_SAFE_INTEGER: usize = u64::MAX as usize;
pub const MIN_SAFE_INTEGER: isize = i64::MIN as isize;
pub const MAX_VALUE: f64 = f64::MAX;
pub const MIN_VALUE: f64 = f64::MIN;

pub const MAX_SAFE_INTEGER_DIGITS: isize = 20; // 18_446_744_073_709_551_615
pub const MIN_SAFE_INTEGER_DIGITS: isize = -19; // -9_223_372_036_854_775_808
pub const MAX_VALUE_DIGITS: isize = f64::MAX_10_EXP as isize;
pub const MIN_VALUE_DIGITS: isize = f64::MIN_10_EXP as isize;

pub fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
pub fn value_undefined() -> Value {
    json!(null)
}
pub fn value_nan() -> Value {
    json!(f64::NAN)
}
pub fn value_infinity() -> Value {
    json!(f64::INFINITY)
}
pub fn number_nan() -> Number {
    // Number::from_f64(f64::NAN).unwrap()
    Number::from(0)
}
pub fn number_infinity() -> Number {
    Number::from_f64(f64::INFINITY).unwrap()
}
pub fn number_neg_infinity() -> Number {
    Number::from_f64(f64::NEG_INFINITY).unwrap()
}
pub fn string_to_option_number(s: String) -> Option<Number> {
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
pub fn vec_value_to_option_number(vec: Vec<Value>) -> Option<Number> {
    match vec.len() {
        0 => Some(value_null_to_number()),
        1 => value_to_option_number(vec[0].clone()),
        _ => None,
    }
}
pub fn value_null_to_number() -> Number {
    Number::from(0)
}
pub fn bool_to_number(b: bool) -> Number {
    if b {
        Number::from(1)
    } else {
        Number::from(0)
    }
}
pub fn value_to_option_number(value: Value) -> Option<Number> {
    match value {
        Value::Null => Some(value_null_to_number()),
        Value::Bool(b) => Some(bool_to_number(b)),
        Value::Number(n) => Some(n),
        Value::String(s) => string_to_option_number(s),
        Value::Array(vec) => vec_value_to_option_number(vec),
        Value::Object(_) => None,
    }
}

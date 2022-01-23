use crate::lib::{json, Value, Number};
use crate::internal::{value_nan, value_to_option_number, vec_value_to_option_number};
use crate::{to_string_x, json_array_to_string_x};

///
pub fn x_add_x(n: Number, n2: Number) -> Number {
    if n.is_u64() {
        let inner_n = n.as_u64().unwrap();
        if n2.is_u64() {
            (inner_n + n2.as_u64().unwrap()).into()
        } else if n2.is_i64() {
            (inner_n as i64 + n2.as_i64().unwrap()).into()
        } else {
            Number::from_f64(inner_n as f64 + n2.as_f64().unwrap()).unwrap()
        }
    } else if n.is_i64() {
        let inner_n = n.as_i64().unwrap();
        if n2.is_u64() {
            (inner_n as u64 + n2.as_u64().unwrap()).into()
        } else if n2.is_i64() {
            (inner_n + n2.as_i64().unwrap()).into()
        } else {
            Number::from_f64(inner_n as f64 + n2.as_f64().unwrap()).unwrap()
        }
    } else {
        let inner_n = n.as_f64().unwrap();
        if n2.is_u64() {
            (inner_n as u64 + n2.as_u64().unwrap()).into()
        } else if n2.is_i64() {
            (inner_n as i64 + n2.as_i64().unwrap()).into()
        } else {
            Number::from_f64(inner_n + n2.as_f64().unwrap()).unwrap()
        }
    }
}

fn x_add_bool_x(n: Number, b: bool) -> Number {
    if n.is_u64() {
        let v: u64 = if b { 1 } else { 0 };
        (n.as_u64().unwrap() + v).into()
    } else if n.is_i64() {
        let v: i64 = if b { 1 } else { 0 };
        (n.as_i64().unwrap() + v).into()
    } else {
        let v: f64 = if b { 1.0 } else { 0.0 };
        Number::from_f64(n.as_f64().unwrap() + v).unwrap()
    }
}
fn array_to_value_number(vec: Vec<Value>) -> Value {
    match vec_value_to_option_number(vec) {
        Some(n) => Value::Number(n),
        None => value_nan(),
    }
}
fn value_to_value_number(value: Value) -> Value {
    match value_to_option_number(value) {
        Some(n) => Value::Number(n),
        None => value_nan(),
    }
}

/// See lodash [add](https://lodash.com/docs/#add)
pub fn add(augend: Value, addend: Value) -> Value {
    match augend {
        Value::Null => match addend {
            Value::Null => json!(0),
            Value::Bool(b) => {
                if b {
                    json!(1)
                } else {
                    json!(0)
                }
            }
            Value::Number(_) => addend,
            Value::String(s) => json!(format!("null{}", s)),
            Value::Array(_) => value_to_value_number(addend),
            Value::Object(_) => value_nan(),
        },
        Value::Bool(b) => match addend {
            Value::Null => json!(0),
            Value::Bool(b2) => {
                if b && b2 {
                    json!(2)
                } else if b || b2 {
                    json!(1)
                } else {
                    json!(0)
                }
            }
            Value::Number(n) => Value::Number(x_add_bool_x(n, b)),
            Value::String(s) => {
                json!(format!("{}{}", b, s))
            }
            Value::Array(vec) => {
                if b {
                    if let Some(n) = vec_value_to_option_number(vec) {
                        Value::Number(x_add_bool_x(n, b))
                    } else {
                        value_nan()
                    }
                } else {
                    array_to_value_number(vec)
                }
            }
            Value::Object(_) => value_nan(),
        },
        Value::Number(n) => match addend {
            Value::Null => Value::Number(n),
            Value::Bool(b) => Value::Number(x_add_bool_x(n, b)),
            Value::Number(n2) => json!(x_add_x(n, n2)),
            Value::String(s) => json!(format!("{}{}", n, s)),
            Value::Array(vec) => match vec.len() {
                0 => Value::Number(n),
                1 => {
                    if let Some(n2) = vec_value_to_option_number(vec) {
                        Value::Number(x_add_x(n, n2))
                    } else {
                        value_nan()
                    }
                }
                _ => value_nan(),
            },
            Value::Object(_) => value_nan(),
        },
        Value::String(mut s) => {
            s.push_str(&to_string_x(addend));
            Value::String(s)
        }
        Value::Array(vec) => match addend {
            Value::Null => array_to_value_number(vec),
            Value::Bool(b) => {
                if b {
                    if let Some(n) = vec_value_to_option_number(vec) {
                        Value::Number(x_add_bool_x(n, b))
                    } else {
                        value_nan()
                    }
                } else {
                    array_to_value_number(vec)
                }
            }
            Value::Number(n) => match vec.len() {
                0 => Value::Number(n),
                1 => {
                    if let Some(n2) = vec_value_to_option_number(vec) {
                        Value::Number(x_add_x(n2, n))
                    } else {
                        value_nan()
                    }
                }
                _ => value_nan(),
            },
            Value::String(s) => Value::String(format!("{}{}", json_array_to_string_x(vec), s)),
            Value::Array(vec2) => {
                if let Some(n) = vec_value_to_option_number(vec) {
                    if let Some(n2) = vec_value_to_option_number(vec2) {
                        return Value::Number(x_add_x(n, n2));
                    }
                }
                value_nan()
            }
            Value::Object(_) => value_nan(),
        },
        Value::Object(_) => value_nan(),
    }
}

/// Based on [x_add_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::Number;
/// assert_eq!(
///   x_add_x!(Number::from(6), Number::from(4)),
///   Number::from(10)
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// ```
#[macro_export]
macro_rules! x_add_x {
    () => {
        Number::from(0)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_add_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_add_x($a, $b)
    };
}
/// Based on [add()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   add!(json!(6), json!(4)),
///   json!(10)
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(add!(), json!(0));
/// assert_eq!(add!(json!(null)), json!(null));
/// assert_eq!(add!(json!(null),json!(null)), json!(0));
/// assert_eq!(add!(json!(null),json!(true)), json!(1));
/// assert_eq!(add!(json!(null),json!(1)), json!(1));
/// assert_eq!(add!(json!(null),json!("_a")), json!("null_a"));
/// assert_eq!(add!(json!(null),json!([])), json!(0));
/// assert_eq!(add!(json!(null),json!([2])), json!(2));
/// assert_eq!(add!(json!(null),json!(["a"])), json!(null)); // NaN
/// assert_eq!(add!(json!(false)), json!(false));
/// assert_eq!(add!(json!(true),json!(false)), json!(1));
/// assert_eq!(add!(json!(true),json!(1.1)), json!(2.1));
/// assert_eq!(add!(json!(false),json!("_b")), json!("false_b"));
/// assert_eq!(add!(json!(true),json!([])), json!(1));
/// assert_eq!(add!(json!(true),json!([2])), json!(3));
/// assert_eq!(add!(json!(true),json!(["a"])), json!(null)); // NaN
/// assert_eq!(add!(json!(0)), json!(0));
/// assert_eq!(add!(json!(1),json!(true)), json!(2));
/// assert_eq!(add!(json!(2),json!("3")), json!("23"));
/// assert_eq!(add!(json!(3),json!([])), json!(3));
/// assert_eq!(add!(json!(4),json!([1])), json!(5));
/// assert_eq!(add!(json!(5),json!([""])), json!(5));
/// assert_eq!(add!(json!(6),json!(["a"])), json!(null)); // NaN
/// assert_eq!(add!(json!("")), json!(""));
/// assert_eq!(add!(json!("a"),json!("b")), json!("ab"));
/// assert_eq!(add!(json!([])), json!([]));
/// assert_eq!(add!(json!([1]),json!([2])), json!(3));
/// assert_eq!(add!(json!({})), json!({}));
/// assert_eq!(add!(json!({}),json!(1)), json!(null)); // NaN
/// ```
#[macro_export]
macro_rules! add {
    () => {
        json!(0)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::add($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::add($a, $b)
    };
}

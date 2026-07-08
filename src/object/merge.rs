use crate::lib::{Map, Value};

// The helpers consume their inputs so the recursion never clones: object
// subtrees are taken out with mem::take and source values are moved in.

// Per-slot merge rule shared by keyed and indexed recursion. Mirrors lodash
// baseMergeDeep: array/object pairs merge recursively, an object slot with
// numeric-keyed object source merges by index, everything else (including an
// object slot receiving an array) is replaced by the source value.
fn merge_value(ov: Value, sv: Value) -> Value {
    match (ov, sv) {
        (Value::Array(oa), Value::Array(sa)) => merge_2_array(oa, sa),
        (Value::Object(om), Value::Object(sm)) => merge_2_object(om, sm),
        (Value::Array(oa), Value::Object(sm)) => merge_array_object(oa, sm),
        (_, sv) => sv,
    }
}

fn merge_2_array(object_vec: Vec<Value>, source_vec: Vec<Value>) -> Value {
    if object_vec.is_empty() {
        return Value::Array(source_vec);
    }
    if source_vec.is_empty() {
        return Value::Array(object_vec);
    }
    let mut object_it = object_vec.into_iter();
    let mut new_v = vec![];
    for v in source_vec {
        new_v.push(match object_it.next() {
            Some(ov) => merge_value(ov, v),
            None => v,
        });
    }
    // like lodash, elements beyond the source's length are kept
    new_v.extend(object_it);
    Value::Array(new_v)
}

fn merge_2_object(mut object: Map<String, Value>, source: Map<String, Value>) -> Value {
    for (k, sv) in source {
        match object.get_mut(&k) {
            Some(ov) => {
                let owned = std::mem::take(ov);
                *ov = merge_value(owned, sv);
            }
            None => {
                object.insert(k, sv);
            }
        }
    }
    Value::Object(object)
}

// Array destination + object source: numeric keys merge by index (extending
// with nulls where lodash would leave holes); keys that aren't valid indexes
// have no JSON representation on an array and are dropped, matching what
// `JSON.stringify` shows for the lodash result.
fn merge_array_object(mut object_vec: Vec<Value>, source: Map<String, Value>) -> Value {
    for (k, sv) in source {
        if let Ok(i) = k.parse::<usize>() {
            if i < object_vec.len() {
                let owned = std::mem::take(&mut object_vec[i]);
                object_vec[i] = merge_value(owned, sv);
            } else {
                object_vec.resize(i + 1, Value::Null);
                object_vec[i] = sv;
            }
        }
    }
    Value::Array(object_vec)
}

/// Fn form of [merge!](crate::merge!); see it for the full docs
///
/// `_x` form: **not provided** — see [merge_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::merge;
/// # use serde_json::json;
/// assert_eq!(merge(json!({"a": 1}), json!({"b": 2})), json!({"a": 1, "b": 2}));
/// ```
pub fn merge(object: Value, source: Value) -> Value {
    match (object, source) {
        (Value::Object(o), Value::Object(s)) => merge_2_object(o, s),
        (Value::Array(o), Value::Array(s)) => merge_2_array(o, s),
        // at the root the destination object survives (lodash mutates it in
        // place), so a source array merges in under its index keys
        (Value::Object(o), Value::Array(s)) => merge_2_object(
            o,
            s.into_iter()
                .enumerate()
                .map(|(i, v)| (i.to_string(), v))
                .collect(),
        ),
        (Value::Array(o), Value::Object(s)) => merge_array_object(o, s),
        (object, _) => object,
    }
}

/// See lodash [merge](https://lodash.com/docs/#merge)
///
/// Fn form: [merge()] | `_x` form: **not provided** — see [merge_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
///
/// let object = json!({
///   "a": [{ "b": 2 }, { "d": 4 }]
/// });
///
/// let other = json!({
///   "a": [{ "c": 3 }, { "e": 5 }]
/// });
///
/// assert_eq!(
///   merge!(object, other),
///   json!({ 'a': [{ 'b': 2, 'c': 3 }, { 'd': 4, 'e': 5 }] })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(merge!(), json!({}));
/// assert_eq!(merge!(json!({'a':1})), json!({'a':1}));
/// # assert_eq!(
/// #   merge!(json!({'a':1}), json!({'b':2}), ),
/// #   json!({'a':1, 'b':2})
/// # );
/// assert_eq!(
///   merge!(json!({'a':1}), json!({'b':2}), json!({'c':3})),
///   json!({'a': 1, 'b': 2, 'c': 3})
/// );
/// # assert_eq!(
/// #   merge!(json!({'a':1}), json!({'b':2}), json!({'c':3}), ),
/// #   json!({'a': 1, 'b': 2, 'c': 3})
/// # );
/// // elements beyond the source's length are kept
/// assert_eq!(merge!(json!([1, 2, 3]), json!([4])), json!([4, 2, 3]));
/// assert_eq!(merge!(json!([{"a":1},{"b":2},3]), json!([{"c":9}])), json!([{"a":1,"c":9},{"b":2},3]));
/// // a source array merges into an object under its index keys
/// assert_eq!(merge!(json!({"a": 1}), json!([9, 8])), json!({"0": 9, "1": 8, "a": 1}));
/// // a numeric-keyed source object merges into an array by index
/// assert_eq!(merge!(json!([1, 2, 3]), json!({"1": 9})), json!([1, 9, 3]));
/// assert_eq!(merge!(json!([1]), json!({"2": 5})), json!([1, null, 5]));
/// assert_eq!(merge!(json!([1]), json!({"a": 2})), json!([1])); // non-index keys dropped
/// // nested: at a key an array source replaces an object value,
/// // and a numeric-keyed object merges into an array value
/// assert_eq!(merge!(json!({"a": {"x": 1}}), json!({"a": [9]})), json!({"a": [9]}));
/// assert_eq!(merge!(json!({"a": [1, 2]}), json!({"a": {"1": 9}})), json!({"a": [1, 9]}));
/// ```
#[macro_export]
macro_rules! merge {
    () => (
        $crate::lib::json!({})
    );
    ($a:expr $(,)*) => {
        $crate::merge($a, $crate::lib::json!({}))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::merge($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::merge!($crate::merge($a, $b), $($rest)*)
    };
}

build_not_provided_x!(merge, merge_x);

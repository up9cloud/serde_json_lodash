//! Curated benches — not blanket coverage. A function earns a bench here when
//! there is a real implementation choice to guard or quantify: the array set
//! ops are O(n²) scans (JSON numbers aren't hashable), sorted_index is a
//! binary search, merge/defaults_deep/clone_deep recurse over nested values,
//! get/set re-parse their path on every call, sort_by/order_by pay a
//! comparator per element, and camel_case exercises the shared words_vec
//! casing core. When optimizing a function, add its bench first as a baseline.

use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use serde_json::{Map, Number, Value, json};
use serde_json_lodash as l;

/// `n` elements drawn from `0..n/2`, so about half are duplicates.
fn int_array_with_dups(n: usize) -> Value {
    Value::Array((0..n).map(|i| json!((i * 7919) % (n / 2))).collect())
}

/// A nested object: `breadth` keys per level, `depth` levels, `leaf` at the bottom.
fn deep_object(depth: usize, breadth: usize, leaf: i64) -> Value {
    if depth == 0 {
        return json!(leaf);
    }
    let mut m = Map::new();
    for i in 0..breadth {
        m.insert(format!("k{i}"), deep_object(depth - 1, breadth, leaf));
    }
    Value::Object(m)
}

pub fn criterion_capitalize(c: &mut Criterion) {
    c.bench_function("::capitalize examples", |b| {
        b.iter(|| l::capitalize(black_box(json!("FRED"))))
    });
    c.bench_function("::capitalize null", |b| {
        b.iter(|| l::capitalize(black_box(json!(null))))
    });
    c.bench_function("::capitalize bool", |b| {
        b.iter(|| l::capitalize(black_box(json!(true))))
    });
    c.bench_function("::capitalize number", |b| {
        b.iter(|| l::capitalize(black_box(json!(-0.1))))
    });
    c.bench_function("::capitalize string", |b| {
        b.iter(|| l::capitalize(black_box(json!("abc"))))
    });
    c.bench_function("::capitalize array", |b| {
        b.iter(|| l::capitalize(black_box(json!([true, -0.1, "abc", [], {}]))))
    });
    c.bench_function("::capitalize object", |b| {
        b.iter(|| l::capitalize(black_box(json!({"a": 123}))))
    });
}

pub fn criterion_extract_value_number(c: &mut Criterion) {
    fn is_as(n: Number) -> Number {
        if n.is_u64() {
            (n.as_u64().unwrap() + 1).into()
        } else if n.is_i64() {
            (n.as_i64().unwrap() + 1).into()
        } else {
            Number::from_f64(n.as_f64().unwrap() + 1.0).unwrap()
        }
    }
    fn let_some_as(n: Number) -> Number {
        if let Some(v) = n.as_u64() {
            (v + 1).into()
        } else if let Some(v) = n.as_i64() {
            (v + 1).into()
        } else {
            Number::from_f64(n.as_f64().unwrap() + 1.0).unwrap()
        }
    }

    let mut group = c.benchmark_group("extract_value_number");
    for (n, k) in [
        (Number::from(1), "u64"),
        (Number::from(-1), "i64"),
        (Number::from_f64(0.0).unwrap(), "f64"),
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("is_* => as_*", k), n, |b, i| {
            b.iter(|| is_as(black_box(i.clone())))
        });
        group.bench_with_input(BenchmarkId::new("let Some(n) = as_*", k), n, |b, i| {
            b.iter(|| let_some_as(black_box(i.clone())))
        });
    }
    group.finish();
}

// O(n²) candidates: element-by-element scans because Value has no cheap hash.
pub fn criterion_array_set_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_set_ops");
    for n in [100usize, 1000] {
        let a = int_array_with_dups(n);
        let b2 = int_array_with_dups(n);
        group.bench_with_input(BenchmarkId::new("intersection", n), &n, |b, _| {
            b.iter_batched(
                || (a.clone(), b2.clone()),
                |(x, y)| l::intersection(x, y),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("uniq", n), &n, |b, _| {
            b.iter_batched(|| a.clone(), l::uniq, BatchSize::SmallInput)
        });
        group.bench_with_input(BenchmarkId::new("union", n), &n, |b, _| {
            b.iter_batched(
                || (a.clone(), b2.clone()),
                |(x, y)| l::union(x, y),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

// The search itself is a binary search (O(log n)), but the public API consumes
// the owned array, so every call pays an O(n) drop of it — that linear floor
// (~1ns/element) is what dominates here and is inherent to the Into<Value>
// API, not the algorithm. If this ever needs to be faster, the fix is a
// by-reference variant, not a different search.
pub fn criterion_sorted_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorted_index");
    for n in [1000usize, 100_000] {
        let sorted = Value::Array((0..n).map(|i| json!(i)).collect());
        let needle = json!(n / 2);
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter_batched(
                || (sorted.clone(), needle.clone()),
                |(arr, v)| l::sorted_index(arr, v),
                BatchSize::LargeInput,
            )
        });
    }
    group.finish();
}

// Recursion over nested values (depth 4 × breadth 4 = 256 leaves).
pub fn criterion_deep_ops(c: &mut Criterion) {
    let a = deep_object(4, 4, 1);
    let b2 = deep_object(4, 4, 2);
    c.bench_function("::merge deep 4x4", |b| {
        b.iter_batched(
            || (a.clone(), b2.clone()),
            |(x, y)| l::merge(x, y),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("::defaults_deep 4x4", |b| {
        b.iter_batched(
            || (a.clone(), b2.clone()),
            |(x, y)| l::defaults_deep(x, y),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("::clone_deep 4x4", |b| {
        b.iter(|| l::clone_deep(black_box(&a)))
    });
}

// Comparator-per-element cost on 1000 pseudo-shuffled numbers.
pub fn criterion_sort(c: &mut Criterion) {
    fn ident(v: &Value) -> Value {
        v.clone()
    }
    let arr = int_array_with_dups(1000);
    c.bench_function("::sort_by 1000", |b| {
        b.iter_batched(
            || arr.clone(),
            |a| l::sort_by(a, ident),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("::order_by 1000 desc", |b| {
        b.iter_batched(
            || arr.clone(),
            |a| l::order_by(a, ident, false),
            BatchSize::SmallInput,
        )
    });
}

// get/set re-parse the string path on every call; this prices that in.
pub fn criterion_path_ops(c: &mut Criterion) {
    let obj = deep_object(4, 4, 1);
    c.bench_function("::get k0.k0.k0.k0", |b| {
        b.iter_batched(
            || obj.clone(),
            |o| l::get(o, json!("k0.k0.k0.k0"), json!(null)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("::set k0.k0.k0.k0", |b| {
        b.iter_batched(
            || obj.clone(),
            |o| l::set(o, json!("k0.k0.k0.k0"), json!(9)),
            BatchSize::SmallInput,
        )
    });
}

// One casing fn covers the shared words_vec/compound_words core.
pub fn criterion_string_casing(c: &mut Criterion) {
    c.bench_function("::camel_case mixed", |b| {
        b.iter(|| l::camel_case(black_box("__FOO_BAR_baz quxQuux--12ab3__")))
    });
}

criterion_group!(
    benches,
    criterion_capitalize,
    criterion_extract_value_number,
    criterion_array_set_ops,
    criterion_sorted_index,
    criterion_deep_ops,
    criterion_sort,
    criterion_path_ops,
    criterion_string_casing
);
criterion_main!(benches);

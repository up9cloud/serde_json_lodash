use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use serde_json::{json, Number};
use serde_json_lodash as l;

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

criterion_group!(
    benches,
    criterion_capitalize,
    criterion_extract_value_number
);
criterion_main!(benches);

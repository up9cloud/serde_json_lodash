use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use serde_json_lodash as lib;

pub fn criterion_capitalize(c: &mut Criterion) {
    c.bench_function("::capitalize examples", |b| {
        b.iter(|| lib::capitalize(black_box(json!("FRED"))))
    });
    c.bench_function("::capitalize null", |b| {
        b.iter(|| lib::capitalize(black_box(json!(null))))
    });
    c.bench_function("::capitalize bool", |b| {
        b.iter(|| lib::capitalize(black_box(json!(true))))
    });
    c.bench_function("::capitalize number", |b| {
        b.iter(|| lib::capitalize(black_box(json!(-0.1))))
    });
    c.bench_function("::capitalize string", |b| {
        b.iter(|| lib::capitalize(black_box(json!("abc"))))
    });
    c.bench_function("::capitalize array", |b| {
        b.iter(|| lib::capitalize(black_box(json!([true, -0.1, "abc", [], {}]))))
    });
    c.bench_function("::capitalize object", |b| {
        b.iter(|| lib::capitalize(black_box(json!({"a": 123}))))
    });
}

criterion_group!(benches, criterion_capitalize);
criterion_main!(benches);

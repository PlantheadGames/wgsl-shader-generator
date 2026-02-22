
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use wgsl_shader_generator::*;
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test_add", |b| b.iter(|| test_add(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

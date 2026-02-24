
use criterion::{criterion_group, criterion_main, Criterion};
//use std::hint::black_box;
use wgsl_shader_generator::*;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test_add", |b| b.iter(|| one_add_one()));
    c.bench_function("test_generation", |b| b.iter(|| criterion_inital_graph_generation()));
    c.bench_function("test_node_linking", |b| b.iter(|| criterion_node_connections()));
}



criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

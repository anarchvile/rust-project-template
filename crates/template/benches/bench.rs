//! Benchmarks for the template library.

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

/// Benchmark addition.
fn bench_add(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter(|| template::add(std::hint::black_box(2), std::hint::black_box(3)));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_add
}
criterion_main!(benches);

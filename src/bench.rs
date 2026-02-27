use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path="./parse.rs"] mod parse;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create tokens - literal", |b| b.iter(|| parse::get_tokens("1")));
    c.bench_function("create tokens - single proc", |b| b.iter(|| parse::get_tokens("(+ 1 1)")));
    c.bench_function("create tokens - nested proc", |b| b.iter(|| parse::get_tokens("(* (let ((x 2))) (* x 2))")));
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.99).sample_size(2000);
    targets = criterion_benchmark
}
criterion_main!(benches);

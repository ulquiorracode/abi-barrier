use abi_barrier::prelude::*;
use core::hint::black_box;
use criterion::{Criterion, criterion_group, criterion_main};

#[barrier(convention = CallingConvention::C, on_panic = PanicStrategy::ReturnDefault)]
fn safe_add(a: i32, b: i32) -> i32 {
    a + b
}

fn raw_add(a: i32, b: i32) -> i32 {
    a + b
}

fn bench_barrier_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("AbiBarrier/Overhead");

    group.bench_function("raw_add", |b| {
        b.iter(|| raw_add(black_box(10), black_box(20)))
    });

    group.bench_function("barrier_safe_add", |b| {
        b.iter(|| safe_add(black_box(10), black_box(20)))
    });

    group.finish();
}

criterion_group!(benches, bench_barrier_overhead);
criterion_main!(benches);

use std::collections::{BTreeSet, HashSet};
use std::hint::black_box;
use criterion::{criterion_group, BenchmarkId, Criterion};

use data_structures::SmartSet;

fn bench_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains");

    for size in [100, 1_000, 10_000, 100_000] {
        let data: Vec<i32> = (0..size).collect();

        let smartset: SmartSet<i32> = data.iter().copied().collect();
        let hashset: HashSet<i32> = data.iter().copied().collect();
        let btreeset: BTreeSet<i32> = data.iter().copied().collect();

        let target = size / 2;

        group.bench_with_input(BenchmarkId::new("smartset", size), &size, |b, _| {
            b.iter(|| { black_box(smartset.contains(&target)) });
        });

        group.bench_with_input(BenchmarkId::new("hashset", size), &size, |b, _| {
            b.iter(|| { black_box(hashset.contains(&target)) });
        });

        group.bench_with_input(BenchmarkId::new("btreeset", size), &size, |b, _| {
            b.iter(|| { black_box(btreeset.contains(&target)) });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_contains);
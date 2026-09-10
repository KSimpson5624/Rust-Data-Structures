use std::collections::HashSet;
use std::hint::black_box;
use criterion::{criterion_group, BatchSize, BenchmarkId, Criterion};
use rand::RngExt;

use data_structures::SmartSet;

fn bench_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort");

    for size in [1, 100, 1_000, 10_000, 100_000] {
        let data: Vec<i32> = (0..size).collect();

        group.bench_with_input(BenchmarkId::new("smartset", size), &size, |b, _| {
            b.iter_batched(||
                data.iter().copied().collect::<SmartSet<i32>>(),
                           |mut set| black_box(set.sort()),
                           BatchSize::SmallInput
            );
        });

        group.bench_with_input(BenchmarkId::new("hashset", size), &size, |b, _| {
            b.iter_batched(||
                data.iter().copied().collect::<HashSet<i32>>(),
                |set| {
                    let mut vec: Vec<i32> = set.iter().copied().collect();
                    vec.sort();
                    black_box(vec);
            },
            BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_sort_before_and_after_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_twice");
    let mut rng = rand::rng();

    for size in [1, 100, 1_000, 10_000, 100_000] {
        // This creates a vector of random i32 values (positive and negative) of the length `size`
        let data: Vec<i32> = (0..size).map(|_| rng.random()).collect();

        let random_insert: i32 = rng.random();


        group.bench_with_input(BenchmarkId::new("smartset", size), &size, |b, _| {
            b.iter_batched(||
                               data.iter().copied().collect::<SmartSet<i32>>(),
                           |mut set| {
                               black_box(set.sort());
                               set.insert(random_insert);
                               black_box(set.sort());
                           },
                           BatchSize::SmallInput
            );
        });

        group.bench_with_input(BenchmarkId::new("hashset", size), &size, |b, _| {
            b.iter_batched(||
                               data.iter().copied().collect::<HashSet<i32>>(),
                           |mut set| {
                               // To sort a HashSet after insertion, it must be copied a second time to a vector.
                               let mut vec: Vec<i32> = set.iter().copied().collect();
                               vec.sort();
                               set.insert(random_insert);
                               vec = set.iter().copied().collect();
                               vec.sort();
                               black_box(vec);
                           },
                           BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}
fn bench_sort_before_and_after_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_twice");

    for size in [1, 100, 1_000, 10_000, 100_000] {
        let data: Vec<i32> = (0..size).collect();

        // This will take half the vector but round down. Therefore, it is safe for vectors of odd values and of size 1.
        let item_removed: i32 = (data.len() / 2) as i32;

        group.bench_with_input(BenchmarkId::new("smartset", size), &size, |b, _| {
            b.iter_batched(||
                               data.iter().copied().collect::<SmartSet<i32>>(),
                           |mut set| {
                               black_box(set.sort());
                               set.remove(&item_removed);
                               black_box(set.sort());
                           },
                           BatchSize::SmallInput
            );
        });

        group.bench_with_input(BenchmarkId::new("hashset", size), &size, |b, _| {
            b.iter_batched(||
                               data.iter().copied().collect::<HashSet<i32>>(),
                           |mut set| {
                               // To sort a HashSet after insertion, it must be copied a second time to a vector.
                               let mut vec: Vec<i32> = set.iter().copied().collect();
                               vec.sort();
                               set.remove(&item_removed);
                               vec = set.iter().copied().collect();
                               vec.sort();
                               black_box(vec);
                           },
                           BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sort, bench_sort_before_and_after_insert, bench_sort_before_and_after_remove);
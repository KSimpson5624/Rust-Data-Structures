
mod membership;
mod sort;

use criterion::criterion_main;

criterion_main!(membership::benches, sort::benches);
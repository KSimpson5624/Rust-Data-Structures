
mod membership;

use criterion::criterion_main;

criterion_main!(membership::benches);
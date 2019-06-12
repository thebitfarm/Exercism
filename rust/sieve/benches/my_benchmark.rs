#[macro_use]
extern crate criterion;
extern crate sieve;

use criterion::Criterion;
use criterion::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sieve(1000)", |b| b.iter(|| sieve::primes_up_to(1000) ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_euler::maths::*;
use std::time::Duration;

/// benchmark functions
///
/// To run all benchmarks use this
///
/// ```rust,ignore
/// cargo bench
/// ```
///
/// To benchmark a single function, use this
///
/// ```rust,ignore
/// cargo bench -- <function name>
/// ````
///
/// example:
///
/// ```rust,ignore
/// cargo bench -- sieve_of_eratosthenes
/// ```
///
/// * Add test cases to `black_box` as required
/// * Change `sample_size`, `measurement_time` and `warm_up_time` as required.
///
/// While running benchmark, sometimes cargo might ask you to increase `measurement_time`.
///

fn benchmark_num_divisors(c: &mut Criterion) {
    let nums = black_box([23, 57]);
    for num in nums {
        c.bench_function(&format!("num_divisors : {}", num), |b| {
            b.iter(|| num_divisors(num))
        });
    }
}

fn benchmark_primes_upto_n(c: &mut Criterion) {
    let nums = black_box([23, 89]);
    for num in nums {
        c.bench_function(&format!("sieve_of_eratosthenes : {}", num), |b| {
            b.iter(|| sieve_of_eratosthenes(num))
        });
        c.bench_function(&format!("primes_upto_n_without_sieve : {}", num), |b| {
            b.iter(|| primes_upto_n_without_sieve(num))
        });
    }
}

criterion_group!(
    name=benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(5))
        .warm_up_time(Duration::from_secs(2));

    targets =
        benchmark_num_divisors,
        benchmark_primes_upto_n,
);
criterion_main!(benches);

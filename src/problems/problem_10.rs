#[allow(unused_imports)]
use project_euler::math;
use std::time::Instant;

#[allow(dead_code)]
fn solution_1(limit: u128) {
    let primes_under_limit: Vec<u128> = math::sieve_of_eratosthenes(limit - 1);
    println!("Answer : {}", primes_under_limit.iter().sum::<u128>());
}

#[allow(dead_code)]
pub fn solve() {
    let limit: u128 = 2_000_000;

    let now: Instant = Instant::now();
    solution_1(limit);
    println!("Elapsed : {:?}", now.elapsed());
}

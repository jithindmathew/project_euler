// https://projecteuler.net/problem=10

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(limit: u128) {
    let primes_under_limit: Vec<u128> = maths::sieve_of_eratosthenes(limit - 1);
    println!("Answer : {}", primes_under_limit.iter().sum::<u128>());
}

#[allow(dead_code)]
pub fn solve() {
    let limit: u128 = 2_000_000;

    time_solutions!(solution_1(limit));
}

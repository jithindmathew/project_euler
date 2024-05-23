// https://projecteuler.net/problem=3

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(number: u128) {
    let mut ans: u128 = maths::int_sqrt(number);

    while ans > 2 {
        if number % ans == 0 && maths::is_prime(ans) {
            println!("Answer : {}", ans);

            return;
        }
        ans -= 1;
    }
}

#[allow(dead_code)]
fn solution_2(number: u128) {
    let ans: u128 = maths::int_sqrt(number);

    let primes: Vec<u128> = maths::sieve_of_eratosthenes(ans);

    for prime in primes.iter().rev() {
        if number % *prime == 0 {
            println!("Answer : {}", prime);
            return;
        }
    }
}

#[allow(dead_code)]
pub fn solve() {
    let number: u128 = 600851475143;

    time_solutions!(solution_1(number), solution_2(number));
}

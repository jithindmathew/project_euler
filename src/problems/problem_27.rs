// https://projecteuler.net/problem=27

#[allow(unused_imports)]
use crate::{maths, time_solutions};

fn get_prime_count(a: i128, b: i128) -> u128 {
    let mut n: i128 = 0;

    loop {
        if n == b {
            break;
        }

        if n > 1 && a % n == 0 && b % n == 0 {
            break;
        }

        if n == 0 && b < 0 {
            break;
        }

        let calculated_prime: i128 = n * n + a * n + b;

        if calculated_prime <= 1 {
            break;
        }

        if !maths::is_prime(calculated_prime as u128) {
            break;
        }

        n += 1;
    }

    return n as u128;
}

#[allow(dead_code)]
fn solution_1() {
    let mut max_consecutive_primes: u128 = 0;
    let mut a_for_max_consecutive_primes: i128 = 0;
    let mut b_for_max_consecutive_primes: i128 = 0;

    for a in -1000..=1000 {
        for b in -1000..=1000 {
            let prime_count: u128 = get_prime_count(a, b);

            if prime_count > max_consecutive_primes {
                max_consecutive_primes = prime_count;
                a_for_max_consecutive_primes = a;
                b_for_max_consecutive_primes = b;
            }
        }
    }

    println!(
        "n : {}, a : {}, b : {}, a * b : {}",
        max_consecutive_primes,
        a_for_max_consecutive_primes,
        b_for_max_consecutive_primes,
        a_for_max_consecutive_primes * b_for_max_consecutive_primes
    );
}

#[allow(dead_code)]
pub fn solve() {
    time_solutions!(solution_1());
}

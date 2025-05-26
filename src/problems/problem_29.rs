// https://projecteuler.net/problem=29

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(limit: u128) {
    let primes_upto_n: Vec<u128> = maths::primes_upto_n_without_sieve(limit);

    let mut tracker_hashset: HashSet<Vec<u128>> = HashSet::new();

    for base in 2..=limit {
        for power in 2..=limit {
            let prime_factors_base: HashMap<u128, u128> =
                maths::prime_factors_of_n_with_sieve_as_hashmap(base);

            let mut unique_representation: Vec<u128> = Vec::new();

            for prime in &primes_upto_n {
                if prime_factors_base.contains_key(prime) {
                    unique_representation.push(1);
                } else {
                    unique_representation.push(0);
                }
            }

            for prime in &primes_upto_n {
                let prime_count_in_power: u128 = *prime_factors_base.get(prime).unwrap_or(&0);

                unique_representation.push(prime_count_in_power * power);
            }

            if !tracker_hashset.contains(&unique_representation) {
                tracker_hashset.insert(unique_representation);
            }
        }
    }

    println!("{}", tracker_hashset.len());
}

#[allow(dead_code)]
pub fn solve() {
    let limit: u128 = 100;
    time_solutions!(solution_1(limit));
}

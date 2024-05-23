// https://projecteuler.net/problem=5

use crate::{maths, time_solutions};
#[allow(unused_imports)]
use std::{collections::HashMap, time::Instant};

#[allow(dead_code)]
fn solution_1(limit: u128) {
    let mut primes_map: HashMap<u128, u128> = HashMap::new();

    let mut ans: u128 = 1;

    let primes: Vec<u128> = maths::sieve_of_eratosthenes(limit);

    for i in &primes {
        primes_map.insert(*i, 0);
    }

    for i in 2..=limit {
        let temp_map: HashMap<u128, u128> = maths::prime_factors_of_n_with_sieve_as_hashmap(i);
        for (&key, &val) in temp_map.iter() {
            primes_map.insert(key, u128::max(val, *primes_map.get(&key).unwrap()));
        }
    }

    for (&key, &val) in primes_map.iter() {
        ans *= u128::pow(key, val as u32);
    }

    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    let limit: u128 = 10;

    time_solutions!(solution_1(limit));
}

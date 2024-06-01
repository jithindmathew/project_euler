// https://projecteuler.net/problem=23

#[allow(unused_imports)]
use crate::{maths, time_solutions};
use std::collections::HashSet;

#[allow(dead_code)]
fn solution_1() {
    let mut ans: u128 = 0;
    let mut abundant_numbers: HashSet<u128> = HashSet::new();

    for num in 2..=28123 {
        let sum_divisors: u128 = maths::sum_of_all_divisors(num as u128) - num;

        if sum_divisors > num {
            abundant_numbers.insert(num);
        }
    }

    for num1 in 1..28123 {
        for num2 in 1..28123 {
            if
        }
    }
    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    time_solutions!(solution_1());
}

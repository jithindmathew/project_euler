// https://projecteuler.net/problem=23

#[allow(unused_imports)]
use crate::{maths, time_solutions};
use std::collections::HashSet;

#[allow(dead_code)]
fn solution_1() {
    let mut ans: u128 = 0;
    let mut abundant_numbers: HashSet<u128> = HashSet::new();
    let mut abundant_numbers_sums: HashSet<u128> = HashSet::new();


    for num in 2..=28123 {
        let sum_divisors: u128 = maths::sum_of_all_divisors(num as u128) - num;

        if sum_divisors > num {
            abundant_numbers.insert(num);
        }
    }

    for num1 in &abundant_numbers {
        for num2 in &abundant_numbers {
            if (num1 + num2) < 28124 {
                abundant_numbers_sums.insert(num1 + num2);
            }
        }
    }

    for num in 1..=28123 {
        if !abundant_numbers_sums.contains(&num) {
            ans += num;
        }
    }

    println!("Answer : {}", ans);

}

#[allow(dead_code)]
pub fn solve() {
    time_solutions!(solution_1());
}

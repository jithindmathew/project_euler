// https://projecteuler.net/problem=12

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1() {
    let mut n: u128 = 1;

    while n < maths::int_sqrt(std::u128::MAX) {
        let triangle_num: u128 = n * (n + 1) / 2;

        if maths::num_divisors(triangle_num) > 500 {
            println!("Answer : {}", triangle_num);
            return;
        }

        n += 1;
    }
}

#[allow(dead_code)]
pub fn solve() {
    time_solutions!(solution_1());
}

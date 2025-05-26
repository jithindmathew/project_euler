// https://projecteuler.net/problem=26

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(limit: u128) {
    let mut max_digits: usize = 0;
    let mut num_with_max_digits: u128 = 2;

    for num in 2..=limit {
        let (_, _, len_decimal_expansion) = maths::decimal_expansion(num);

        if len_decimal_expansion > max_digits {
            max_digits = len_decimal_expansion;
            num_with_max_digits = num;
        }
    }

    println!(
        "Number : {} has the most number of repeating digits : {}",
        num_with_max_digits, max_digits
    );
}

#[allow(dead_code)]
pub fn solve() {
    let limit: u128 = 1000;
    time_solutions!(solution_1(limit));
}

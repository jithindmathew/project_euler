// https://projecteuler.net/problem=30

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn digit_power_sum(num: u128) -> u128 {
    let mut digits: Vec<u128> = Vec::new();

    let mut num: u128 = num;

    while num > 0 {
        let digit: u128 = num % 10;
        digits.push(digit);
        num /= 10;
    }

    let mut power_sum: u128 = 0;

    for digit in digits {
        power_sum += digit.pow(5);
    }

    return power_sum;
}

#[allow(dead_code)]
fn solution_1(upper_limit: u128) {
    let mut sum_of_satisfying_numbers: u128 = 0;

    for num in 2..=upper_limit {
        if num == digit_power_sum(num) {
            sum_of_satisfying_numbers += num;
        }
    }

    println!("Sum : {}", sum_of_satisfying_numbers);
}

#[allow(dead_code)]
pub fn solve() {
    let upper_limit: u128 = 6 * 9_u128.pow(5); // https://www.youtube.com/watch?v=yaEhy5X9qcI
    time_solutions!(solution_1(upper_limit));
}

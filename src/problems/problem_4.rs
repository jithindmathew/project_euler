// https://projecteuler.net/problem=4

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(n_digit: u32) {
    let upper: u128 = u128::pow(10, n_digit) - 1;
    let lower: u128 = u128::pow(10, n_digit - 1);

    let mut ans: u128 = 0;

    for num1 in (lower..=upper).rev() {
        for num2 in (lower..=upper).rev() {
            let product: u128 = (num1 * num2) as u128;
            if maths::is_palindrome(product) {
                ans = u128::max(ans, product);
            }
        }
    }
    println!("Answer : {}", ans)
}

#[allow(dead_code)]
pub fn solve() {
    let n_digit: u32 = 3;

    time_solutions!(solution_1(n_digit));
}

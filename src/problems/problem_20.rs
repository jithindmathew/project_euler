// https://projecteuler.net/problem=20

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(num: u128) {
    let mut ans_vec: Vec<u8> = vec![1];

    for num in 1..=num {
        let num1: Vec<u8> = maths::u128_to_vecu8(num);
        ans_vec = maths::multiply_two_numbers_as_vec(num1, ans_vec);
    }

    let ans: u128 = ans_vec.iter().map(|&x| x as u128).sum::<u128>();

    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    let num: u128 = 100;
    time_solutions!(solution_1(num));
}

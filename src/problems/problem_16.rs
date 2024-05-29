// https://projecteuler.net/problem=16

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(power: u128, base: u128) {
    let power_vec: Vec<u8> = maths::u128_to_vecu8(power);
    let base_vec: Vec<u8> = maths::u128_to_vecu8(base);

    let ans_vec: Vec<u8> = maths::get_power_of_a_number(base_vec, power_vec);
    println!("Answer : {:?}", ans_vec);

    let ans: u128 = ans_vec.iter().map(|&x| x as u128).sum::<u128>();

    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    let base: u128 = 2;
    let power: u128 = 1000;
    time_solutions!(solution_1(power, base));
}

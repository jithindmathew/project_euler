// https://projecteuler.net/problem=24

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(digit: u128, position: u128) {
    let mut digit: u128 = digit;
    let mut position: u128 = position - 1;
    let mut available_digits: Vec<u8> = (0..=digit).map(|x: u128| x as u8).collect::<Vec<u8>>();
    let mut ans: Vec<u8> = Vec::new();

    digit += 1;

    while digit > 0 {
        let quotient: u128 = position / maths::factorial_as_u128(digit - 1);
        let remainder: u128 = position % maths::factorial_as_u128(digit - 1);

        ans.push(available_digits[quotient as usize]);

        available_digits.remove(quotient as usize);

        position = remainder;

        digit -= 1;
    }
    let ans_str: String = ans.iter().map(|&x| (x + b'0') as char).collect::<String>();

    println!("{}", ans_str);

    println!(
        "{:?}",
        maths::add_two_numbers_as_vec(vec![1, 2], vec![1, 2],)
    );
}

#[allow(dead_code)]
pub fn solve() {
    let digit: u128 = 9;
    let position: u128 = 1_000_000;
    time_solutions!(solution_1(digit, position));
}

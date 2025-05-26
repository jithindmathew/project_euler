// https://projecteuler.net/problem=25

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(num_digits: u128) {
    let mut fib_n_minus_2: Vec<u8> = vec![1];
    let mut fib_n_minus_1: Vec<u8> = vec![1];
    let mut fib_n: Vec<u8>;

    let mut index: u128 = 2;

    loop {
        fib_n = maths::add_two_numbers_as_vec(fib_n_minus_1.clone(), fib_n_minus_2.clone(),);

        index += 1;

        if fib_n.len() >= num_digits as usize {
            break;
        }

        fib_n_minus_2 = fib_n_minus_1.clone();
        fib_n_minus_1 = fib_n.clone();
    }
    println!("{}", index);
}

#[allow(dead_code)]
pub fn solve() {
    let num_digits: u128 = 1000;
    time_solutions!(solution_1(num_digits));
}

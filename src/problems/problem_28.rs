// https://projecteuler.net/problem=28

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(side_length: u128) {
    let matrix: Vec<Vec<u128>> = maths::create_spiral_matrix(side_length);

    let mut diagonal_sum: u128 = 0;

    for i in 0..side_length as usize {
        diagonal_sum += matrix[i][i];
        diagonal_sum += matrix[i][side_length as usize - i - 1];
    }

    println!("{:?}", diagonal_sum - 1);
}

#[allow(dead_code)]
pub fn solve() {
    let side_length: u128 = 1001;
    time_solutions!(solution_1(side_length));
}

// https://projecteuler.net/problem=15

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(n: u128) -> () {
    let ans: u128 =
        maths::construct_number_from_prime_factor_hashmap(maths::combinations(2 * n, n));
    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() -> () {
    let grid_side: u128 = 20;
    time_solutions!(solution_1(grid_side));
}

#[allow(unused_imports)]
use crate::{
    maths,
    time_solutions,
};

#[allow(dead_code)]
fn solution_1(n: u128) {
    println!("Answer : {}", maths::nth_prime(n));
}

#[allow(dead_code)]
pub fn solve() {
    let n: u128 = 10001;

    time_solutions!(
        solution_1(n)
    );
}

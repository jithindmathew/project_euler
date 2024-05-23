// https://projecteuler.net/problem=1

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(limit: u128, multiples_of: Vec<u128>) {
    let mut ans: u128 = 0;

    for i in 0..limit {
        for num in &multiples_of {
            if i % *num == 0 {
                ans += i;
                break;
            }
        }
    }
    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    let limit: u128 = 1000;
    let multiples_of: Vec<u128> = vec![3, 5];

    time_solutions!(solution_1(limit, multiples_of));
}

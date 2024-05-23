// https://projecteuler.net/problem=2

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(limit: u128) {
    let mut a: u128 = 1;
    let mut b: u128 = 2;
    let mut c: u128 = a + b;

    let mut ans: u128 = b;

    while c <= limit {
        c = a + b;
        if c % 2 == 0 {
            ans += c;
        }
        a = b;
        b = c;
    }
    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    let limit: u128 = 4_000_000;

    time_solutions!(solution_1(limit));
}

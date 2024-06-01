#[allow(unused_imports)]
use crate::{maths, time_solutions};

pub fn solve() {
    println!("PROBLEM NOT SOLVED");

    for i in 1..=100 {
        println!("{} {:?}", i, maths::sum_of_all_divisors(i) - i);
    }
}


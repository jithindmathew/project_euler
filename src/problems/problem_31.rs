// https://projecteuler.net/problem=31

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(target: u128, available_counts: [u128; 8]) {
    if target == 0 {
        println!("Count : 1");
        return;
    }

    if available_counts.len() == 0 {
        println!("Count : 0");
        return;
    }

    let mut dp: Vec<u128> = vec![0; (target + 1) as usize];
    dp[0] = 1;

    for count in available_counts {
        for i in count..=target {
            dp[i as usize] += dp[(i - count) as usize];
        }
    }
    println!("Count : {}", dp.last().unwrap());
}

#[allow(dead_code)]
pub fn solve() {
    let target: u128 = 200;
    let available_chunks: [u128; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    time_solutions!(solution_1(target, available_chunks));
}

#[allow(unused_imports)]
use project_euler::math;
use std::time::Instant;

#[allow(dead_code)]
fn solution_1(n: u128) {
    println!(
        "Answer : {}",
        u128::pow((n * (n + 1)) / 2, 2) - (n * (n + 1) * (2 * n + 1)) / 6
    );
}

#[allow(dead_code)]
pub fn solve() {
    let n: u128 = 100;

    let now: Instant = Instant::now();
    solution_1(n);
    println!("Elapsed : {:?}", now.elapsed());
}

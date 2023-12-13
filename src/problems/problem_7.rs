#[allow(unused_imports)]
use project_euler::math;
use std::time::Instant;

#[allow(dead_code)]
fn solution_1() {
    println!("Answer : {}", math::nth_prime(10001));
}

#[allow(dead_code)]
pub fn solve() {
    let now: Instant = Instant::now();
    solution_1();
    println!("Elapsed : {:?}", now.elapsed());
}

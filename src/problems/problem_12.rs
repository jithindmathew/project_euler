#[allow(unused_imports)]
use project_euler::math;
use std::time::Instant;

#[allow(dead_code)]
fn solution_1() {
    let mut n: u128 = 1;

    while n < math::int_sqrt(std::u128::MAX) {
        let triangle_num: u128 = n * (n + 1) / 2;

        if math::num_divisors(triangle_num) > 500 {
            println!("Answer : {}", triangle_num);
            return;
        }

        n += 1;
    }
}

#[allow(dead_code)]
pub fn solve() {
    let now: Instant = Instant::now();
    solution_1();
    println!("Elapsed : {:?}", now.elapsed());
}

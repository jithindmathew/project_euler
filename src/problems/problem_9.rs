#[allow(unused_imports)]
use project_euler::math;
use std::time::Instant;

#[allow(dead_code)]
fn solution_1(n: u128) {
    /*
    Euclid's Formula

    For 2 positive integers m, n where (m > n)
    Pythagorean triplets are given by

    a = m^2 - n^2
    b = 2*m*n
    c = m^2 + n^2
    */
    let sqrt: u128 = math::int_sqrt(n) + 1;

    for n in 1..=sqrt {
        for m in n + 1..=sqrt {
            let a: u128 = m * m - n * n;
            let b: u128 = 2 * m * n;
            let c: u128 = m * m + n * n;

            if a + b + c == 1000 {
                println!("Answer : {}", a * b * c);
                return;
            }
        }
    }
}

#[allow(dead_code)]
pub fn solve() {
    let n: u128 = 1000;

    let now: Instant = Instant::now();
    solution_1(n);
    println!("Elapsed : {:?}", now.elapsed());
}

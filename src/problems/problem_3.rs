#[allow(unused_imports)]
use project_euler::math;
use std::time::Instant;

#[allow(dead_code)]
fn solution_1(number: u128) {
    let mut ans: u128 = math::int_sqrt(number);

    while ans > 2 {
        if number % ans == 0 && math::is_prime(ans) {
            println!("Answer : {}", ans);

            return;
        }
        ans -= 1;
    }
}

#[allow(dead_code)]
fn solution_2(number: u128) {
    let ans: u128 = math::int_sqrt(number);

    let primes: Vec<u128> = math::sieve_of_eratosthenes(ans);

    for prime in primes.iter().rev() {
        if number % *prime == 0 {
            println!("{}", prime);
            return;
        }
    }
}

#[allow(dead_code)]
pub fn solve() {
    let number: u128 = 600851475143;

    let now: Instant = Instant::now();
    solution_1(number);
    println!("Elapsed : {:?}", now.elapsed());
}

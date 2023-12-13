#[allow(unused_imports)]
use project_euler::math;
use std::time::Instant;

fn is_palindrome(n: u128) -> bool {
    let mut reversed: u128 = 0;
    let mut number: u128 = n;

    while number > 0 {
        reversed = reversed * 10 + number % 10;
        number = number / 10;
    }

    return reversed == n;
}

#[allow(dead_code)]
fn solution_1(n_digit: u32) {
    let upper: u128 = u128::pow(10, n_digit) - 1;
    let lower: u128 = u128::pow(10, n_digit - 1);

    let mut ans: u128 = 0;

    for num1 in (lower..=upper).rev() {
        for num2 in (lower..=upper).rev() {
            let product: u128 = (num1 * num2) as u128;
            if is_palindrome(product) {
                ans = u128::max(ans, product);
            }
        }
    }
    println!("Answer : {}", ans)
}

#[allow(dead_code)]
pub fn solve() {
    let n_digit: u32 = 3;

    let now: Instant = Instant::now();
    solution_1(n_digit);
    println!("Elapsed : {:?}", now.elapsed());
}

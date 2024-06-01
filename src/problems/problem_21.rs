// https://projecteuler.net/problem=21

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(num: u128) {
    let mut ans: u128 = 0;
    let mut amicable_map: Vec<u128> = vec![0; num as usize];

    for n in 1..=num {
        amicable_map[(n - 1) as usize] = maths::sum_of_all_divisors(n) - n;
    }

    for n1 in 1..=num {
        for n2 in n1..=num {
            if (n1 == amicable_map[(n2 - 1) as usize])
                && (n2 == amicable_map[(n1 - 1) as usize])
                && (n1 != n2)
            {
                ans += n1 + n2;
            }
        }
    }

    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    let num: u128 = 10000;
    time_solutions!(solution_1(num));
}

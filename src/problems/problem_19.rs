// https://projecteuler.net/problem=19

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1() {
    let mut day_num: u8 = 1;
    let mut ans: u128 = 0;

    for year in 1900..=2000 {
        for month in 1..=12 {
            let days: u32 = match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                2 => {
                    if ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0) {
                        29
                    } else {
                        28
                    }
                }
                _ => 30,
            };

            for day in 1..=days {
                if (day == 1) && (year >= 1901) && (year <= 2000) && (day_num == 7) {
                    ans += 1;
                }

                match day_num {
                    1..=6 => day_num = day_num + 1,
                    7 => day_num = 1,
                    _ => unreachable!(),
                }
            }
        }
    }
    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    time_solutions!(solution_1());
}

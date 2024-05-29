// https://projecteuler.net/problem=17

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1() {
    let ones_map: [&str; 20] = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let tens_map: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut ans: usize = 0;

    for num in 1..20 {
        ans += ones_map[num as usize].len();
        // println!("{}", ones_map[num as usize]);
    }

    for num in 20..100 {
        let numm: usize = num;
        let ones_digit: usize = num % 10;
        let tens_digit: usize = numm / 10;

        ans += ones_map[ones_digit].len();
        ans += tens_map[tens_digit].len();

        // println!("{}{}", tens_map[tens_digit], ones_map[ones_digit]);
    }

    for num in 100..1000 {
        let mut numm: usize = num;
        let ones_digit: usize = numm % 10;
        numm /= 10;
        let tens_digit: usize = numm % 10;
        let hundreds_digits: usize = numm / 10;

        if tens_digit < 2 {
            let tens_ones_number: usize = tens_digit * 10 + ones_digit;
            ans += ones_map[hundreds_digits].len() + "hundred".len();

            if tens_ones_number == 0 {
                // println!("{}{}", ones_map[hundreds_digits], "hundred");
                continue;
            }
            ans += "and".len() + ones_map[tens_ones_number].len();
            // println!("{}{}{}{}", ones_map[hundreds_digits], "hundred", "and", ones_map[tens_ones_number]);
        } else {
            ans += ones_map[hundreds_digits].len() + "hundred".len() + "and".len();
            ans += tens_map[tens_digit].len();
            ans += ones_map[ones_digit].len();

            // println!("{}{}{}{}{}", ones_map[hundreds_digits], "hundred", "and", tens_map[tens_digit], ones_map[ones_digit]);
        }
    }

    ans += "onethousand".len();

    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    time_solutions!(solution_1());
}

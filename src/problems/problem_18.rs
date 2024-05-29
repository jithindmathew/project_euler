// https://projecteuler.net/problem=18

#[allow(unused_imports)]
use crate::{maths, time_solutions};

#[allow(dead_code)]
fn solution_1(pyramid: Vec<Vec<u128>>) {
    let depth: usize = pyramid.len();

    fn helper(
        pyramid: &Vec<Vec<u128>>,
        vertical_level: usize,
        horizontal_level: usize,
        depth: &usize,
        curr_sum: u128,
    ) -> u128 {
        if vertical_level == *depth - 1 {
            return curr_sum;
        }

        return std::cmp::max(
            helper(
                &pyramid,
                vertical_level + 1,
                horizontal_level,
                &depth,
                curr_sum + pyramid[vertical_level + 1][horizontal_level],
            ),
            helper(
                &pyramid,
                vertical_level + 1,
                horizontal_level + 1,
                &depth,
                curr_sum + pyramid[vertical_level + 1][horizontal_level + 1],
            ),
        );
    }

    let ans: u128 = helper(&pyramid, 0, 0, &depth, pyramid[0][0]);

    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    let pyramid: Vec<Vec<u128>> = vec![
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 04, 82, 47, 65],
        vec![19, 01, 23, 75, 03, 34],
        vec![88, 02, 77, 73, 07, 63, 67],
        vec![99, 65, 04, 28, 06, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
    ];

    time_solutions!(solution_1(pyramid));
}

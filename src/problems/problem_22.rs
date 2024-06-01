// https://projecteuler.net/problem=22

#[allow(unused_imports)]
use crate::{maths, time_solutions};
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
fn solution_1(file_name: &str) {
    let mut file: File = File::open(file_name).unwrap();
    let mut content: String = String::new();

    let mut ans: u128 = 0;

    file.read_to_string(&mut content).unwrap();

    let mut items: Vec<&str> = content.trim().trim_matches('"').split("\",\"").collect();

    items.sort();

    for (index, &name) in items.iter().enumerate() {
        ans += ((index + 1) as u128)
            * name
                .chars()
                .map(|c| ((c as u8) - ('A' as u8) + 1) as u128)
                .sum::<u128>()
    }

    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    let file_name: &str = "0022_names.txt";
    let file_path_string: String = format!("files/{}", file_name);
    let file_path: &str = &file_path_string;

    time_solutions!(solution_1(file_path));
}

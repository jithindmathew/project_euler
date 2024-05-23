// https://projecteuler.net/problem=14

use crate::time_solutions;
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(dead_code)]
fn solution_1(limit: u128) {
    // uses hashmap to store the sequence_lengths already created
    fn helper(map: &mut HashMap<u128, u128>, number: u128, limit: u128) -> u128 {
        #[allow(unused_assignments)]
        let mut sequence_length: u128 = 1;

        if map.contains_key(&number) {
            sequence_length = *map.get(&number).unwrap();

            return sequence_length;
        } else {
            if (number & 1) == 0 {
                sequence_length = helper(map, number / 2, limit) + 1;

                if number <= 3 * limit {
                    map.insert(number, sequence_length);
                }
                // map.insert(number, sequence_length);

                return sequence_length;
            } else {
                sequence_length = helper(map, 3 * number + 1, limit) + 1;

                if number <= 3 * limit {
                    map.insert(number, sequence_length);
                }
                // map.insert(number, sequence_length);

                return sequence_length;
            }
        }
    }

    let mut mapp: HashMap<u128, u128> = HashMap::from([(1, 1)]);

    let mut number: u128 = 1;
    let mut ans: u128 = number;

    let mut longest_sequence: u128 = *mapp.get(&number).unwrap();

    while number < limit {
        let result: u128 = helper(&mut mapp, number, limit);

        if result > longest_sequence {
            ans = number;
            longest_sequence = result;
        }

        number += 1;
    }

    println!("Using Hashmap");
    println!("Answer : {}", ans);
}

#[allow(dead_code)]
pub fn solve() {
    let limit: u128 = 1_000_000;

    time_solutions!(solution_1(limit));
}

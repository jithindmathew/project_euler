use std::env;
mod macros;
mod maths;
mod problems;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid Command");
        return;
    }

    if args[1] == "all" {
        todo!();
    }

    let problem_numbers: Vec<u32> = args[1..]
        .iter()
        .map(|num: &String| num.parse::<u32>().unwrap())
        .collect();

    for problem in problem_numbers {
        println!("=============================================================================");
        println!("Problem : {}", problem);
        match problem {
            1 => problems::problem_1::solve(),
            2 => problems::problem_2::solve(),
            3 => problems::problem_3::solve(),
            4 => problems::problem_4::solve(),
            5 => problems::problem_5::solve(),
            6 => problems::problem_6::solve(),
            7 => problems::problem_7::solve(),
            8 => problems::problem_8::solve(),
            9 => problems::problem_9::solve(),
            10 => problems::problem_10::solve(),
            11 => problems::problem_11::solve(),
            12 => problems::problem_12::solve(),
            13 => problems::problem_13::solve(),
            14 => problems::problem_14::solve(),
            15 => problems::problem_15::solve(),
            16 => problems::problem_16::solve(),
            17 => problems::problem_17::solve(),
            18 => problems::problem_18::solve(),
            19 => problems::problem_19::solve(),
            20 => problems::problem_20::solve(),
            21 => problems::problem_21::solve(),
            22 => problems::problem_22::solve(),
            23 => problems::problem_23::solve(),
            24 => problems::problem_24::solve(),
            25 => problems::problem_25::solve(),
            26 => problems::problem_26::solve(),
            27 => problems::problem_27::solve(),
            28 => problems::problem_28::solve(),
            29 => problems::problem_29::solve(),
            30 => problems::problem_30::solve(),
            31 => problems::problem_31::solve(),
            _ => println!("Problem Not Solved"),
        };
    }
    println!("=============================================================================")
}

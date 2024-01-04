use std::fs;

use day09::new_last_number;

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(result) => result,
        Err(msg) => {
            panic!("File could not be read: {msg}");
        }
    };

    let result: i32 = input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num_str| num_str.parse::<i32>())
                .filter_map(|x| x.ok())
                .collect();

            new_last_number(&numbers)
        })
        .sum();

    println!("Sum of next numbers of sequences is: {}", result);
}

use std::fs;

use day09::new_last_number;

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(result) => result,
        Err(msg) => {
            panic!("File could not be read: {msg}");
        }
    };

    let input_numbers: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>())
                .filter_map(|x| x.ok())
                .collect()
        })
        .collect();

    // Part 1

    let last_num_sum: i32 = input_numbers.iter().map(|v| new_last_number(v)).sum();

    println!("Sum of next numbers of sequences is: {}", last_num_sum);

    // Part 2

    // The algorithm works for extrapolating the past by just reversing the input...

    let input_numbers_reversed: Vec<Vec<i32>> = input_numbers
        .iter()
        .map(|v| v.iter().map(|i| i.to_owned()).rev().collect())
        .collect();

    let first_num_sum: i32 = input_numbers_reversed
        .iter()
        .map(|v| new_last_number(v))
        .sum();

    println!(
        "Sum of extrapolated first numbers of sequences is: {}",
        first_num_sum
    );
}

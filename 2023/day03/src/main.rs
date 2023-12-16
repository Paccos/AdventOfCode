use day03::{parse_numbers, parse_parts, sum_numbers_next_to_parts, sum_of_gear_ratios};
use std::fs;

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(result) => result,
        Err(msg) => {
            panic!("File could not be read: {msg}");
        }
    };

    let parts = parse_parts(&input);
    let numbers = parse_numbers(&input);

    let max_x = input.lines().next().unwrap().len();
    let max_y = input.lines().count();

    let sum_of_pt_nums = sum_numbers_next_to_parts(&parts, &numbers, max_x, max_y);
    println!("The sum of all part nums is {}", sum_of_pt_nums);

    let sum_of_gear_ratios = sum_of_gear_ratios(&parts, &numbers, max_x, max_y);
    println!("The sum of all gear ratios is {}", sum_of_gear_ratios);
}

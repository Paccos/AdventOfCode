#[cfg(test)]
mod tests {
    use std::fs;

    use day03::{parse_numbers, parse_parts, sum_numbers_next_to_parts};

    #[test]
    fn sum_for_test_input_equals_4361() {
        let test_input = match fs::read_to_string("tests/test-input") {
            Ok(result) => result,
            Err(msg) => {
                panic!("File could not be read: {msg}");
            }
        };

        let max_x = test_input.lines().next().unwrap().len();
        let max_y = test_input.lines().count();

        let numbers = parse_numbers(&test_input);
        let parts = parse_parts(&test_input);

        let sum = sum_numbers_next_to_parts(&parts, &numbers, max_x, max_y);

        assert_eq!(sum, 4361);
    }
}

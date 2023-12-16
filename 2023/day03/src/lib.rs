use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Number {
    number: i32,
    line: usize,
    start_col: usize,
    len: usize,
}

#[derive(Debug, PartialEq)]
pub struct Part {
    symbol: char,
    coords: (usize, usize),
}

impl Part {
    fn get_neighbors(&self, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
        let (x, y) = self.coords;

        let mut result: Vec<(usize, usize)> = Vec::new();

        // top row
        // only if we're not in the top row already...
        if y != 0 {
            // ...add the top coords as neighbors

            // only if we're not in the first column already...
            if x != 0 {
                result.push((x - 1, y - 1)); // ...add the top left neighbor
            }

            result.push((x, y - 1));

            // only if we're not in the last column already...
            if x != max_x {
                result.push((x + 1, y - 1)) // ...add the top right neighbor
            }
        }

        // left and right neighbors
        // only if we're not in the first column already...
        if x != 0 {
            result.push((x - 1, y)); // ...add the left neighbor
        }

        // only if we're not in the last column already...
        if x != max_x {
            result.push((x + 1, y)) // ...add the right neighbor
        }

        // bottom row
        // only if we're not in the last row already...
        if y != max_y {
            // ...add the bottom coords as neighbors

            // only if we're not in the first column already...
            if x != 0 {
                result.push((x - 1, y + 1)); // ...add the top left neighbor
            }

            result.push((x, y + 1));

            // only if we're not in the last column already...
            if x != max_x {
                result.push((x + 1, y + 1)) // ...add the bottom right neighbor
            }
        }

        result
    }

    fn is_neighbor(&self, num: &Number, max_x: usize, max_y: usize) -> bool {
        let neighbors = self.get_neighbors(max_x, max_y);

        for (x, _y) in neighbors.iter().filter(|(_x, y)| y == &num.line) {
            if x < &(num.start_col + num.len) && x >= &num.start_col {
                return true;
            }
        }

        false
    }

    fn get_numbers<'a>(
        &'a self,
        numbers: &'a [Number],
        max_x: usize,
        max_y: usize,
    ) -> Vec<&Number> {
        let result: Vec<&'a Number> = numbers
            .iter()
            .filter(|n| self.is_neighbor(n, max_x, max_y))
            .collect();

        result
    }

    fn gear_ratio(&self, numbers: &[Number], max_x: usize, max_y: usize) -> i32 {
        if self.symbol != '*' {
            return 0;
        }

        let nums = self.get_numbers(numbers, max_x, max_y);

        if nums.len() != 2 {
            return 0;
        }

        nums[0].number * nums[1].number
    }
}

pub fn sum_numbers_next_to_parts(
    parts: &[Part],
    numbers: &[Number],
    max_x: usize,
    max_y: usize,
) -> i32 {
    parts.iter().fold(0, |sum, p| {
        sum + p
            .get_numbers(numbers, max_x, max_y)
            .iter()
            .map(|n| n.number)
            .sum::<i32>()
    })
}

pub fn sum_of_gear_ratios(parts: &[Part], numbers: &[Number], max_x: usize, max_y: usize) -> i32 {
    parts
        .iter()
        .filter(|p| p.symbol == '*')
        .fold(0, |sum, p| sum + p.gear_ratio(numbers, max_x, max_y))
}

pub fn parse_numbers(input: &str) -> Vec<Number> {
    let lines: Vec<&str> = input.lines().collect();
    let mut result: Vec<Number> = Vec::new();

    let num_re = Regex::new(r"\d+").unwrap();

    for (y, line) in lines.iter().enumerate() {
        for num_match in num_re.find_iter(line) {
            let num_str = num_match.as_str();
            let num = Number {
                number: num_str.parse().unwrap(),
                line: y,
                start_col: num_match.start(),
                len: num_str.len(),
            };

            result.push(num);
        }
    }

    result
}

pub fn parse_parts(input: &str) -> Vec<Part> {
    let lines: Vec<&str> = input.lines().collect();
    let mut result: Vec<Part> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_alphanumeric() && c != '.' {
                result.push(Part {
                    symbol: c,
                    coords: (x, y),
                });
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn neighbors_for_top_left_part_only_has_right_coords() {
        let part = Part {
            symbol: '*',
            coords: (0, 0),
        };

        let result = part.get_neighbors(42, 42);

        assert_eq!(result, vec![(1, 0), (0, 1), (1, 1)]);
    }

    #[test]
    fn neighbors_for_part_in_top_row_does_not_have_top_neighbors() {
        let part = Part {
            symbol: '*',
            coords: (1, 0),
        };

        let result = part.get_neighbors(42, 42);

        assert_eq!(result, vec![(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)]);
    }

    #[test]
    fn neighbors_for_top_right_part_only_has_left_coords() {
        let part = Part {
            symbol: '*',
            coords: (42, 0),
        };

        let result = part.get_neighbors(42, 42);

        assert_eq!(result, vec![(41, 0), (41, 1), (42, 1)]);
    }

    #[test]
    fn neighbors_for_part_in_middle_has_all_coords() {
        let part = Part {
            symbol: '*',
            coords: (12, 13),
        };

        let result = part.get_neighbors(42, 42);

        assert_eq!(
            result,
            vec![
                (11, 12),
                (12, 12),
                (13, 12),
                (11, 13),
                (13, 13),
                (11, 14),
                (12, 14),
                (13, 14)
            ]
        );
    }

    #[test]
    fn neighbors_for_bottom_left_part_only_has_right_coords() {
        let part = Part {
            symbol: '*',
            coords: (0, 42),
        };

        let result = part.get_neighbors(42, 42);

        assert_eq!(result, vec![(0, 41), (1, 41), (1, 42)]);
    }

    #[test]
    fn neighbors_for_bottom_right_part_only_has_left_coords() {
        let part = Part {
            symbol: '*',
            coords: (42, 42),
        };

        let result = part.get_neighbors(42, 42);

        assert_eq!(result, vec![(41, 41), (42, 41), (41, 42)]);
    }

    #[test]
    fn neighbors_for_part_in_bottom_row_does_not_have_bottom_neighbors() {
        let part = Part {
            symbol: '*',
            coords: (12, 42),
        };

        let result = part.get_neighbors(42, 42);

        assert_eq!(
            result,
            vec![(11, 41), (12, 41), (13, 41), (11, 42), (13, 42)]
        );
    }

    #[test]
    fn parse_numbers_finds_all_numbers() {
        let test_input = match fs::read_to_string("tests/test-input") {
            Ok(result) => result,
            Err(msg) => {
                panic!("File could not be read: {msg}");
            }
        };

        let numbers = parse_numbers(&test_input);

        assert_eq!(
            numbers,
            vec![
                Number {
                    number: 467,
                    line: 0,
                    start_col: 0,
                    len: 3
                },
                Number {
                    number: 114,
                    line: 0,
                    start_col: 5,
                    len: 3
                },
                Number {
                    number: 35,
                    line: 2,
                    start_col: 2,
                    len: 2,
                },
                Number {
                    number: 633,
                    line: 2,
                    start_col: 6,
                    len: 3
                },
                Number {
                    number: 617,
                    line: 4,
                    start_col: 0,
                    len: 3
                },
                Number {
                    number: 58,
                    line: 5,
                    start_col: 7,
                    len: 2
                },
                Number {
                    number: 592,
                    line: 6,
                    start_col: 2,
                    len: 3
                },
                Number {
                    number: 755,
                    line: 7,
                    start_col: 6,
                    len: 3
                },
                Number {
                    number: 664,
                    line: 9,
                    start_col: 1,
                    len: 3
                },
                Number {
                    number: 598,
                    line: 9,
                    start_col: 5,
                    len: 3
                }
            ]
        );
    }

    #[test]
    fn parse_parts_finds_all_parts() {
        let test_input = match fs::read_to_string("tests/test-input") {
            Ok(result) => result,
            Err(msg) => {
                panic!("File could not be read: {msg}");
            }
        };

        let parts = parse_parts(&test_input);

        assert_eq!(
            parts,
            vec![
                Part {
                    symbol: '*',
                    coords: (3, 1)
                },
                Part {
                    symbol: '#',
                    coords: (6, 3)
                },
                Part {
                    symbol: '*',
                    coords: (3, 4)
                },
                Part {
                    symbol: '+',
                    coords: (5, 5)
                },
                Part {
                    symbol: '$',
                    coords: (3, 8)
                },
                Part {
                    symbol: '*',
                    coords: (5, 8)
                }
            ]
        )
    }

    #[test]
    fn is_neighbor_is_true_for_top_left() {
        let num = Number {
            number: 42,
            line: 2,
            start_col: 2,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_true_for_top_middle() {
        let num1 = Number {
            number: 42,
            line: 2,
            start_col: 3,
            len: 2,
        };

        let num2 = Number {
            number: 42,
            line: 2,
            start_col: 4,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(part.is_neighbor(&num1, 42, 42));
        assert!(part.is_neighbor(&num2, 42, 42));
    }

    #[test]
    fn is_neighbor_is_true_for_top_right() {
        let num = Number {
            number: 42,
            line: 2,
            start_col: 5,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_true_for_left() {
        let num = Number {
            number: 42,
            line: 3,
            start_col: 2,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_true_for_right() {
        let num = Number {
            number: 42,
            line: 3,
            start_col: 5,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_true_for_bottom_left() {
        let num = Number {
            number: 42,
            line: 4,
            start_col: 2,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_true_for_bottom_middle() {
        let num1 = Number {
            number: 42,
            line: 4,
            start_col: 3,
            len: 2,
        };

        let num2 = Number {
            number: 42,
            line: 4,
            start_col: 4,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(part.is_neighbor(&num1, 42, 42));
        assert!(part.is_neighbor(&num2, 42, 42));
    }

    #[test]
    fn is_neighbor_is_true_for_bottom_right() {
        let num = Number {
            number: 42,
            line: 4,
            start_col: 5,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_false_for_top_left_gap() {
        let num = Number {
            number: 42,
            line: 2,
            start_col: 1,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(!part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_false_for_top_middle_gap() {
        let num1 = Number {
            number: 42,
            line: 1,
            start_col: 3,
            len: 2,
        };

        let num2 = Number {
            number: 42,
            line: 1,
            start_col: 4,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(!part.is_neighbor(&num1, 42, 42));
        assert!(!part.is_neighbor(&num2, 42, 42));
    }

    #[test]
    fn is_neighbor_is_false_for_top_right_gap() {
        let num = Number {
            number: 42,
            line: 2,
            start_col: 6,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(!part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_false_for_left_gap() {
        let num = Number {
            number: 42,
            line: 3,
            start_col: 1,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(!part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_false_for_right_gap() {
        let num = Number {
            number: 42,
            line: 3,
            start_col: 6,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(!part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_false_for_bottom_left_gap() {
        let num = Number {
            number: 42,
            line: 4,
            start_col: 1,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(!part.is_neighbor(&num, 42, 42));
    }

    #[test]
    fn is_neighbor_is_false_for_bottom_middle_gap() {
        let num1 = Number {
            number: 42,
            line: 5,
            start_col: 3,
            len: 2,
        };

        let num2 = Number {
            number: 42,
            line: 5,
            start_col: 4,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(!part.is_neighbor(&num1, 42, 42));
        assert!(!part.is_neighbor(&num2, 42, 42));
    }

    #[test]
    fn is_neighbor_is_false_for_bottom_right_gap() {
        let num = Number {
            number: 42,
            line: 4,
            start_col: 6,
            len: 2,
        };

        let part = Part {
            symbol: '*',
            coords: (4, 3),
        };

        assert!(!part.is_neighbor(&num, 42, 42));
    }
}

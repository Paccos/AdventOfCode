fn diff_chain(nums: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for i in 0..(nums.len() - 1) {
        result.push(nums[i + 1] - nums[i]);
    }

    result
}

fn diff_chain_to_zero(input: &[i32]) -> Vec<Vec<i32>> {
    let mut result = vec![input.to_owned()];
    let mut zeros = input.iter().all(|&x| x == 0);

    while !zeros {
        if let Some(last_list) = result.last() {
            let next = diff_chain(last_list);

            zeros = next.iter().all(|&x| x == 0);

            result.push(next);
        }
    }

    result
}

fn propagate(input: &[Vec<i32>]) -> Vec<i32> {
    let last_value = input
        .iter()
        .fold(0, |acc, next| next.last().unwrap_or(&0) + acc);

    let mut result = input[0].clone();
    result.push(last_value);

    result
}

pub fn new_last_number(input: &[i32]) -> i32 {
    *propagate(&diff_chain_to_zero(input)).last().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn diff_chain_produces_vec_of_threes() {
        let input = vec![0, 3, 6, 9, 12, 15];

        let result = diff_chain(&input);

        assert!(result.iter().all(|&x| x == 3));
        assert_eq!(result.len(), input.len() - 1);
    }

    #[test]
    fn diff_chain_to_zero_reduces_down_to_zero() {
        let input = vec![0, 3, 6, 9, 12, 15];

        let result = diff_chain_to_zero(&input);

        assert_eq!(3, result.len());
        assert!(result.last().unwrap().iter().all(|&x| x == 0));
    }

    #[test]
    fn propagate_returns_correct_sequence() {
        let input = vec![0, 3, 6, 9, 12, 15];

        let propagation_result = propagate(&diff_chain_to_zero(&input));

        assert_eq!(vec![0, 3, 6, 9, 12, 15, 18], propagation_result);
    }

    #[test_case(vec![0, 3, 6, 9, 12, 15], 18)]
    #[test_case(vec![1, 3, 6, 10, 15, 21], 28)]
    #[test_case(vec![10, 13, 16, 21, 30, 45], 68)]
    fn new_last_number_returns_correct_number_for_sequence(input: Vec<i32>, expected: i32) {
        assert_eq!(new_last_number(&input), expected);
    }
}

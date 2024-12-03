advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(to_numbers)
            .filter(|nums| is_safe(nums))
            .count() as u32,
    )
}

fn to_numbers(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|token| token.parse::<i32>().ok())
        .collect()
}

fn is_safe(nums: &[i32]) -> bool {
    let direction = (nums[0] - nums[1]) >> 31;
    nums.windows(2)
        .map(|pair| pair[0] - pair[1])
        .all(|distance| distance >> 31 == direction && (1..=3).contains(&distance.abs()))
}

fn is_safe_with_joker(nums: &Vec<i32>) -> bool {
    is_safe(nums)
        || (0..nums.len())
            .map(|i| list_without_i_th(nums, i))
            .any(|nums_wo_i_th| is_safe(&nums_wo_i_th))
}

fn list_without_i_th(nums: &[i32], i: usize) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .filter(|(j, _)| *j != i)
        .map(|(_, v)| *v)
        .collect::<Vec<i32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(to_numbers)
            .filter(is_safe_with_joker)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(710));
    }

    #[test]
    fn test_is_safe_2_should_allow_to_skip_first_element() {
        // GIVEN
        let nums = vec![1, 4, 5, 6];

        // WHEN
        let res = is_safe_with_joker(&nums);

        // THEN
        assert_eq!(res, true);
    }

    #[test]
    fn test_is_safe_2_should_allow_to_skip_second_element_with_change_direction() {
        // GIVEN
        let nums = vec![9, 15, 8, 7, 6];

        // WHEN
        let res = is_safe_with_joker(&nums);

        // THEN
        assert_eq!(res, true);
    }

    #[test]
    fn test_is_safe_2_should_allow_to_skip_second_element_without_change_of_direction() {
        // GIVEN
        let nums = vec![9, 15, 10, 11, 12];

        // WHEN
        let res = is_safe_with_joker(&nums);

        // THEN
        assert_eq!(res, true);
    }

    #[test]
    fn test_is_safe_2_should_allow_increasing_levels() {
        // GIVEN
        let nums = vec![9, 10, 11, 12, 13];

        // WHEN
        let res = is_safe_with_joker(&nums);

        // THEN
        assert_eq!(res, true);
    }

    #[test]
    fn test_is_safe_2_should_not_allow_two_not_increasing_levels() {
        // GIVEN
        let nums = vec![9, 10, 10, 10, 11];

        // WHEN
        let res = is_safe_with_joker(&nums);

        // THEN
        assert_eq!(res, false);
    }

    #[test]
    fn test_is_safe_2_should_allow_if_last_level_is_wrong() {
        // GIVEN
        let nums = vec![9, 10, 11, 12, 42];

        // WHEN
        let res = is_safe_with_joker(&nums);

        // THEN
        assert_eq!(res, true);
    }

    #[test]
    fn test_is_safe_2_should_not_allow_if_two_first_levels_are_wrong() {
        // GIVEN
        let nums = vec![1, 19, 9, 11, 12];

        // WHEN
        let res = is_safe_with_joker(&nums);

        // THEN
        assert_eq!(res, false);
    }

    #[test]
    fn test_is_safe_2_should_allow_to_skip_second_even_if_working_with_first() {
        // GIVEN
        let nums = vec![59, 61, 57, 55, 54];

        // WHEN
        let res = is_safe_with_joker(&nums);

        // THEN
        assert_eq!(res, true);
    }
}

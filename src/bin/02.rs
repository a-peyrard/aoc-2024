use std::io::BufRead;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .as_bytes()
            .lines()
            .flatten()
            .map(|s| to_numbers(&s))
            .filter_map(is_safe)
            .count() as u32,
    )
}

fn to_numbers(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|token| token.parse::<i32>().ok())
        .collect()
}

fn is_safe(nums: Vec<i32>) -> Option<bool> {
    let first = nums[0];
    let second = nums[1];
    let direction = if first >= second { -1 } else { 1 };
    let distance = (first - second).abs();
    if !(1..=3).contains(&distance) {
        return None;
    }

    let mut latest = second;
    for current in nums.iter().skip(2) {
        let raw_distance = latest - current;
        if direction >= 0 && raw_distance > 0 {
            return None;
        }
        if direction < 0 && raw_distance < 0 {
            return None;
        }
        let distance = raw_distance.abs();
        if !(1..=3).contains(&distance) {
            return None;
        }
        latest = *current;
    }

    Some(true)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

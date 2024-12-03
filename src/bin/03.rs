use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let reg: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(
        reg.captures_iter(input)
            .map(|cap| cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap())
            .sum::<u64>(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(159_892_596));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_should_work_with_3_digits_numbers() {
        // WHEN
        let res = part_one("mul(123,4)");

        // THEN
        assert_eq!(res, Some(492));
    }
}

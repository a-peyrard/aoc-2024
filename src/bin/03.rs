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

pub fn part_two(input: &str) -> Option<u64> {
    let reg: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut activated = true;
    let mut res: u64 = 0;
    for cap in reg.captures_iter(input) {
        match &cap[0] {
            "do()" => activated = true,
            "don't()" => activated = false,
            _ => match activated {
                true => res += cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap(),
                false => {}
            },
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(159_892_596));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_should_work_with_3_digits_numbers() {
        // WHEN
        let res = part_one("mul(123,4)");

        // THEN
        assert_eq!(res, Some(492));
    }

    #[test]
    fn test_should_do_or_in_regex() {
        // GIVEN
        let reg: Regex = Regex::new(r"foo|bar").unwrap();

        // WHEN
        let res = reg.captures_iter("hellofoosomebarisworldfoo").count();

        // THEN
        assert_eq!(res, 3);
    }

    #[test]
    fn test_should_do_or_in_part2_regex() {
        // GIVEN
        let reg: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

        // WHEN
        let res = reg
            .captures_iter(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            )
            .count();

        // THEN
        assert_eq!(res, 6);
    }
}

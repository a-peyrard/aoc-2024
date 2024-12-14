use regex::Regex;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let (num_a, num_b) = input
        .split("\n\n")
        .map(Machine::parse)
        .filter_map(|m| m.solve())
        .fold(
            (0, 0), //
            |(acc_a, acc_b), (cur_a, cur_b)| (acc_a + cur_a, acc_b + cur_b),
        );

    Some(num_a as u32 * 3 + num_b as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

struct Machine {
    prize: (i32, i32),
    a: (i32, i32),
    b: (i32, i32),
}

impl Machine {
    fn parse(input: &str) -> Self {
        let mut tokens = input.split('\n');
        let a = Machine::parse_line(tokens.next().unwrap());
        let b = Machine::parse_line(tokens.next().unwrap());
        let prize = Machine::parse_line(tokens.next().unwrap());

        Self { prize, a, b }
    }

    fn parse_line(line: &str) -> (i32, i32) {
        let line_regex = Regex::new(r".*: X[+=](\d+), Y[+=](\d+)").unwrap();
        let caps = line_regex.captures(line).unwrap();

        (caps[1].parse().unwrap(), caps[2].parse().unwrap())
    }

    fn solve(&self) -> Option<(i32, i32)> {
        let (x, y) = self.prize;
        let (xa, ya) = self.a;
        let (xb, yb) = self.b;

        if let Some(num_b) = divide_int(y * xa - x * ya, yb * xa - xb * ya) {
            if let Some(num_a) = divide_int(x - num_b * xb, xa) {
                return Some((num_a, num_b));
            }
        }

        None
    }
}

fn divide_int(a: i32, b: i32) -> Option<i32> {
    if b == 0 || a % b != 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_one_first_machine() {
        let result = part_one(
            r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
"#,
        );
        assert_eq!(result, Some(280));
    }

    #[test]
    fn test_part_one_second_machine() {
        let result = part_one(
            r#"Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
"#,
        );
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one_third_machine() {
        let result = part_one(
            r#"Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450
"#,
        );
        assert_eq!(result, Some(200));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(part_generic(
        input,
        vec![
            |a, b| a + b, //
            |a, b| a * b, //
        ],
    ))
}

fn part_generic(input: &str, operations: Vec<fn(u64, u64) -> u64>) -> u64 {
    input
        .lines()
        .map(Equation::parse)
        .filter(|eq| eq.solve(&operations))
        .map(|eq| eq.result)
        .sum()
}

struct Equation {
    result: u64,
    components: Vec<u64>,
}

impl Equation {
    pub fn parse(line: &str) -> Self {
        let mut tokens = line.split(": ");
        let result = tokens.next().unwrap().parse::<u64>().unwrap();
        let components = tokens
            .next()
            .unwrap()
            .split_whitespace()
            .map(|e| e.parse().unwrap())
            .collect();

        Self { result, components }
    }

    fn solve(&self, operations: &Vec<fn(u64, u64) -> u64>) -> bool {
        self.solve_rec(
            operations,
            *self.components.first().unwrap(),
            &self.components,
            1,
        )
    }

    fn solve_rec(
        &self,
        operations: &Vec<fn(u64, u64) -> u64>,
        current: u64,
        components: &Vec<u64>,
        index: usize,
    ) -> bool {
        match components.get(index) {
            Some(component) => {
                current <= self.result
                    && operations.iter().any(|op| {
                        self.solve_rec(operations, op(current, *component), components, index + 1)
                    })
            }
            None => current == self.result,
        }
    }
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_one_should_solve_1013() {
        let result = part_one(
            r#"1013: 4 916 93 1
"#,
        );
        assert_eq!(result, Some(1013));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

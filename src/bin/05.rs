use itertools::enumerate;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut blank_line = 0;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for (index, line) in enumerate(input.lines()) {
        if line.is_empty() {
            blank_line = index;
            break;
        }
        let (dependency, page) = parse_rule(line);

        let dependencies = rules.entry(page).or_default();
        dependencies.push(dependency);
    }

    let printer = Printer::new(rules);
    let sum_printable_instructions = input
        .lines()
        .skip(blank_line + 1)
        .map(parse_instruction)
        .filter(|instructions| printer.can_print(instructions))
        .map(|instructions| instructions[instructions.len() / 2])
        .sum();

    Some(sum_printable_instructions)
}

fn parse_rule(line: &str) -> (u32, u32) {
    let mut tokens = line.split('|');

    (
        tokens.next().unwrap().parse::<u32>().unwrap(),
        tokens.next().unwrap().parse::<u32>().unwrap(),
    )
}

fn parse_instruction(line: &str) -> Vec<u32> {
    line.split(',')
        .map(|token| token.parse::<u32>().unwrap())
        .collect()
}

struct Printer {
    rules: HashMap<u32, Vec<u32>>,
}

impl Printer {
    fn new(rules: HashMap<u32, Vec<u32>>) -> Self {
        Self { rules }
    }

    fn can_print(&self, instructions: &Vec<u32>) -> bool {
        let mut printed = HashSet::new();
        let instructions_set = instructions.iter().collect::<HashSet<&u32>>();

        for page in instructions {
            let printable = self
                .rules
                .get(page)
                .map(|dependencies| {
                    dependencies.iter().all(|dependency| {
                        !instructions_set.contains(dependency) || printed.contains(dependency)
                    })
                })
                .unwrap_or(true);

            if !printable {
                return false;
            }

            printed.insert(page);
        }

        true
    }
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

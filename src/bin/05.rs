use itertools::enumerate;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, end_of_rules) = parse_rules(input);

    let printer = Printer::new(&rules);
    let sum_printable_instructions = input
        .lines()
        .skip(end_of_rules + 1)
        .map(parse_instruction)
        .filter(|instructions| printer.can_print(instructions))
        .map(|instructions| instructions[instructions.len() / 2])
        .sum();

    Some(sum_printable_instructions)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, end_of_rules) = parse_rules(input);

    let printer = Printer::new(&rules);

    let sum_printable_instructions = input
        .lines()
        .skip(end_of_rules + 1)
        .map(parse_instruction)
        .filter(|instructions| !printer.can_print(instructions))
        .map(|instructions| re_order(instructions, &rules))
        .map(pick_middle_page)
        .sum();

    Some(sum_printable_instructions)
}

fn pick_middle_page(instructions: Vec<u32>) -> u32 {
    instructions[instructions.len() / 2]
}

fn re_order(instructions: Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let pages_set: HashSet<u32> = instructions.into_iter().collect();

    let mut ordered = Vec::new();
    compute_topological_order_rec(
        find_root(rules, &pages_set),
        rules,
        &pages_set,
        &mut ordered,
        &mut HashSet::<u32>::new(),
    );

    ordered
}

fn compute_topological_order_rec(
    current: u32,
    rules: &HashMap<u32, Vec<u32>>,
    pages_set: &HashSet<u32>,
    ordered: &mut Vec<u32>,
    visited: &mut HashSet<u32>,
) {
    visited.insert(current);
    if let Some(dependencies) = rules.get(&current) {
        for dependency in dependencies.iter().filter(|p| pages_set.contains(p)) {
            if !visited.contains(dependency) {
                compute_topological_order_rec(*dependency, rules, pages_set, ordered, visited);
            }
        }
    }

    ordered.push(current);
}

fn find_root(rules: &HashMap<u32, Vec<u32>>, pages_set: &HashSet<u32>) -> u32 {
    let dependencies: HashSet<u32> = rules
        .iter()
        .filter(|(page, _)| pages_set.contains(page))
        .flat_map(|(_, d)| d.iter())
        .copied()
        .collect();

    *rules
        .keys()
        .filter(|p| pages_set.contains(p))
        .find(|p| !dependencies.contains(p))
        .unwrap()
}

fn parse_rules(input: &str) -> (HashMap<u32, Vec<u32>>, usize) {
    let mut end_of_rules = 0;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for (index, line) in enumerate(input.lines()) {
        if line.is_empty() {
            end_of_rules = index;
            break;
        }
        let (dependency, page) = parse_rule(line);

        let dependencies = rules.entry(page).or_default();
        dependencies.push(dependency);
    }

    (rules, end_of_rules)
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

struct Printer<'a> {
    rules: &'a HashMap<u32, Vec<u32>>,
}

impl<'a> Printer<'a> {
    fn new(rules: &'a HashMap<u32, Vec<u32>>) -> Self {
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
        assert_eq!(result, Some(123));
    }
}

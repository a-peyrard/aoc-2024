use std::collections::HashMap;
use std::str::Lines;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (all_patterns, designs) = parse(input);

    let mut memo = HashMap::<String, bool>::new();
    let count = designs
        .filter(|design| is_possible(design, &all_patterns, &mut memo))
        .count() as u32;

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (all_patterns, designs) = parse(input);

    let mut memo = HashMap::<String, u64>::new();
    let count = designs
        .map(|design| arrangements(design, &all_patterns, &mut memo))
        .sum();

    Some(count)
}

fn parse(input: &str) -> (Patterns, Lines<'_>) {
    let (b1, b2) = input.split_once("\n\n").unwrap();

    let mut all_patterns = Patterns::new();
    b1.lines()
        .next()
        .unwrap()
        .split(", ")
        .for_each(|token| all_patterns.insert(token));

    let lines = b2.lines();

    (all_patterns, lines)
}

#[derive(Default, Debug)]
struct Patterns {
    is_final: bool,
    patterns: HashMap<char, Patterns>,
}

impl Patterns {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: &str) {
        let mut current_node = self;
        for c in word.chars() {
            current_node = current_node.patterns.entry(c).or_default();
        }
        current_node.is_final = true;
    }

    fn get(&self, c: char) -> Option<&Patterns> {
        self.patterns.get(&c)
    }
}

fn is_possible(design: &str, all_patterns: &Patterns, memo: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&cached) = memo.get(design) {
        return cached;
    }

    let mut patterns = all_patterns;
    for (i, cur) in design.chars().enumerate() {
        if let Some(cur_patterns) = patterns.get(cur) {
            patterns = cur_patterns;
        } else {
            memo.insert(design[i..].to_owned(), false);
            return false;
        }

        #[allow(clippy::collapsible_if)]
        if patterns.is_final {
            if is_possible(&design[i + 1..], all_patterns, memo) {
                memo.insert(design[i + 1..].to_owned(), true);
                return true;
            }
        }
    }

    memo.insert(design.to_owned(), false);
    false
}

fn arrangements(design: &str, all_patterns: &Patterns, memo: &mut HashMap<String, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&cached) = memo.get(design) {
        return cached;
    }

    let mut count = 0;
    let mut patterns = all_patterns;
    for (i, cur) in design.chars().enumerate() {
        if let Some(cur_patterns) = patterns.get(cur) {
            patterns = cur_patterns;
        } else {
            break;
        }

        #[allow(clippy::collapsible_if)]
        if patterns.is_final {
            count += arrangements(&design[i + 1..], all_patterns, memo);
        }
    }

    memo.insert(design.to_owned(), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    // #[test]
    // fn test_part_one_input() {
    //     let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    //     assert_eq!(result, Some(290));
    // }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    // #[test]
    // fn test_part_two_input() {
    //     let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    //     assert_eq!(result, Some(16));
    // }
}

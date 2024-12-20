use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (b1, b2) = input.split_once("\n\n").unwrap();

    let mut all_patterns = Patterns::new();
    b1.lines()
        .next()
        .unwrap()
        .split(", ")
        .for_each(|token| all_patterns.insert(token));

    let mut possibles = 0;
    for design in b2.lines() {
        if is_possible(design, &all_patterns) {
            println!("design <{}> is possible!", design);
            possibles += 1;
        } else {
            println!("design <{}> is NOT possible!", design);
        }
    }

    // b2.lines()
    // .filter(|design| is_possible(design.as_bytes(), &patterns))
    // .count() as u32,

    Some(possibles)
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

fn is_possible(design: &str, all_patterns: &Patterns) -> bool {
    if design.is_empty() {
        return true;
    }

    let mut patterns = all_patterns;
    for (i, cur) in design.chars().enumerate() {
        if let Some(cur_patterns) = patterns.get(cur) {
            patterns = cur_patterns;
        } else {
            return false;
        }

        #[allow(clippy::collapsible_if)]
        if patterns.is_final {
            if is_possible(&design[i + 1..], all_patterns) {
                return true;
            }
        }
    }

    false
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
        assert_eq!(result, Some(6));
    }

    // #[test]
    // fn test_part_one_input() {
    //     let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    //     assert_eq!(result, Some(6));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

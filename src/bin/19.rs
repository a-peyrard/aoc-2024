advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (b1, b2) = input.split_once("\n\n").unwrap();

    let patterns: Vec<&[u8]> = b1
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|t| t.as_bytes())
        .collect();

    Some(
        b2.lines()
            .filter(|design| is_possible(design.as_bytes(), &patterns))
            .count() as u32,
    )
}

fn is_possible(design: &[u8], patterns: &[&[u8]]) -> bool {
    if design.is_empty() {
        return true;
    }

    for &pattern in patterns {
        #[allow(clippy::collapsible_if)]
        if can_use(design, pattern) {
            if is_possible(&design[pattern.len()..], patterns) {
                return true;
            }
        }
    }

    false
}

fn can_use(design: &[u8], pattern: &[u8]) -> bool {
    if design.len() < pattern.len() {
        return false;
    }

    for i in 0..pattern.len() {
        if design[i] != pattern[i] {
            return false;
        }
    }

    true
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

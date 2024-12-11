advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .split_whitespace()
            .map(|token| token.parse().unwrap())
            .map(|stone| expand(stone, 25))
            .sum(),
    )
}

fn expand(stone: u64, blink: u32) -> u64 {
    if blink == 0 {
        return 1;
    }

    if stone == 0 {
        expand(1, blink - 1)
    } else if let Some((stone1, stone2)) = split(stone) {
        expand(stone1, blink - 1) + expand(stone2, blink - 1)
    } else {
        expand(stone * 2024, blink - 1)
    }
}

fn split(stone: u64) -> Option<(u64, u64)> {
    let serialized = stone.to_string();

    match serialized.len() % 2 {
        0 => {
            let (s1, s2) = serialized.split_at(serialized.len() / 2);
            Some((s1.parse().unwrap(), s2.parse().unwrap()))
        }
        _ => None,
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
        let result = part_one("125 17");
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("125 17");
        assert_eq!(result, None);
    }
}

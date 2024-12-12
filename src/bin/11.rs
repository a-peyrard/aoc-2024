use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    part_gen(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_gen(input, 75)
}

fn part_gen(input: &str, blink: u32) -> Option<u64> {
    let mut memo = HashMap::<(u64, u32), u64>::new();

    Some(
        input
            .split_whitespace()
            .map(|token| token.parse().unwrap())
            .map(|stone| expand_memo(stone, blink, &mut memo))
            .sum(),
    )
}

fn expand_memo(stone: u64, blink: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if let Some(&cached) = memo.get(&(stone, blink)) {
        return cached;
    }

    if blink == 0 {
        return 1;
    }

    let res = if stone == 0 {
        expand_memo(1, blink - 1, memo)
    } else if let Some((stone1, stone2)) = split(stone) {
        expand_memo(stone1, blink - 1, memo) + expand_memo(stone2, blink - 1, memo)
    } else {
        expand_memo(stone * 2024, blink - 1, memo)
    };

    memo.insert((stone, blink), res);

    res
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
        assert_eq!(result, Some(65601038650482));
    }
}

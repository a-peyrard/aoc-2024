use std::collections::{BinaryHeap, HashMap};
use std::io::BufRead;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let numbers: Vec<_> = input
        .as_bytes()
        .lines()
        .flatten()
        .map(|s| find_numbers(&s))
        .collect();

    let mut first_list = BinaryHeap::new();
    let mut second_list = BinaryHeap::new();

    for (x, y) in numbers.into_iter().flatten() {
        first_list.push(x);
        second_list.push(y);
    }

    let mut res: u32 = 0;
    while !first_list.is_empty() && !second_list.is_empty() {
        if let Some(x) = first_list.pop() {
            if let Some(y) = second_list.pop() {
                res += i32::abs(x - y) as u32;
            }
        }
    }

    Some(res)
}

fn find_numbers(s: &str) -> Option<(i32, i32)> {
    let mut tokens = s.split_whitespace();
    if let (Some(first), Some(second)) = (tokens.next(), tokens.next()) {
        if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
            return Some((num1, num2));
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers: Vec<_> = input
        .as_bytes()
        .lines()
        .flatten()
        .map(|s| find_numbers(&s))
        .collect();

    let mut second_list = HashMap::new();
    for (_, y) in numbers.iter().flatten() {
        *second_list.entry(y).or_insert(0) += 1;
    }

    let mut res: u32 = 0;
    for (x, _) in numbers.iter().flatten() {
        res += (*x * *second_list.entry(x).or_default()) as u32
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

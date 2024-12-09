use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid_size = input.lines().next().unwrap().len() as i32;

    Some(
        input
            .lines()
            .clone()
            .enumerate()
            .flat_map(|(y, line)| Antenna::find(y as i32, line))
            .sorted_by_key(|a| a.label)
            .group_by(|a| a.label)
            .into_iter()
            .flat_map(|(_, group)| {
                group.into_iter().combinations(2).flat_map(|pair| {
                    Antenna::antinodes(pair[0].clone(), pair[1].clone(), grid_size)
                })
            })
            .collect::<HashSet<Antinode>>()
            .len() as u32,
    )
}

#[derive(Clone, Debug)]
struct Antenna {
    label: u8,
    x: i32,
    y: i32,
}

type Antinode = (i32, i32);

impl Antenna {
    fn find(y: i32, line: &str) -> Vec<Self> {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != b'.')
            .map(|(x, &label)| Self {
                label,
                x: x as i32,
                y,
            })
            .collect()
    }

    fn antinodes(a: Antenna, b: Antenna, grid_size: i32) -> Vec<Antinode> {
        let (dx, dy) = (b.x - a.x, b.y - a.y);

        let mut antinodes = Vec::new();
        let after_b = (b.x + dx, b.y + dy);
        if Antenna::in_range(after_b, grid_size) {
            antinodes.push(after_b);
        }
        let before_a = (a.x - dx, a.y - dy);
        if Antenna::in_range(before_a, grid_size) {
            antinodes.push(before_a);
        }

        antinodes
    }

    fn in_range((x, y): (i32, i32), grid_size: i32) -> bool {
        x >= 0 && x < grid_size && y >= 0 && y < grid_size
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(254));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

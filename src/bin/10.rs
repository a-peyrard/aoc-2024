use advent_of_code::util::grid::{Direction, Grid};
use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse_input(input);

    Some(
        grid.find(b'0')
            .map(|coords| find_trails(coords, &grid))
            .sum(),
    )
}

const DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

fn find_trails(start: (usize, usize), grid: &Grid) -> u32 {
    find_trails_rec(start, 0, grid)
        .into_iter()
        .collect::<HashSet<(usize, usize)>>()
        .len() as u32
}

fn find_trails_rec((x, y): (usize, usize), number: u32, grid: &Grid) -> Vec<(usize, usize)> {
    if number == 9 {
        return vec![(x, y)];
    }

    let mut result = Vec::new();
    for &direction in DIRECTIONS {
        if let Some((x, y)) = grid.get_coords(direction, x, y) {
            if grid.elems[y][x] == char::from_digit(number + 1, 10).unwrap() as u8 {
                result.append(&mut find_trails_rec((x, y), number + 1, grid));
            }
        }
    }

    result
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_example_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_example_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_example_4() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_example_5() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, None);
    }
}

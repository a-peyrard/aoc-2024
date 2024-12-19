use advent_of_code::util::grid::{Direction, Grid};
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);

type Coord = (usize, usize);

pub fn part_one(input: &str) -> Option<u32> {
    part_one_gen(input, 71, 1024)
}

fn part_one_gen(input: &str, grid_size: usize, number_of_bytes: usize) -> Option<u32> {
    let mut grid = Grid::new(vec![".".repeat(grid_size); grid_size]);

    input
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .take(number_of_bytes)
        .for_each(|pos| grid.set(pos, b'#'));

    Some(find_shortest_path(&grid))
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn find_shortest_path(grid: &Grid) -> u32 {
    let start = (0, 0);
    let target = (grid.width - 1, grid.height - 1);

    let mut visited = HashSet::<Coord>::new();
    let mut queue = VecDeque::<(Coord, u32)>::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((cur, w)) = queue.pop_front() {
        if cur == target {
            return w;
        }

        for direction in DIRECTIONS {
            if let Some(next) = grid.get_coords2(direction, cur) {
                if !visited.contains(&next) && grid.get(next) == b'.' {
                    queue.push_back((next, w + 1));
                    visited.insert(next);
                }
            }
        }
    }

    0
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_gen(input, 71)
}

pub fn part_two_gen(input: &str, grid_size: usize) -> Option<String> {
    let bytes = input
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect::<Vec<Coord>>();

    let mut high = bytes.len() - 1;
    let mut low = 0;
    while low <= high {
        let mid = low + ((high - low) / 2);

        let mut grid = Grid::new(vec![".".repeat(grid_size); grid_size]);
        bytes.iter().take(mid).for_each(|&pos| grid.set(pos, b'#'));
        match find_shortest_path(&grid) {
            0 => high = mid - 1,
            _ => low = mid + 1,
        }
    }

    let first = bytes[low - 1];
    let mut res = String::new();
    res.push_str(&first.0.to_string());
    res.push(',');
    res.push_str(&first.1.to_string());

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one_gen(&advent_of_code::template::read_file("examples", DAY), 7, 12);
        assert_eq!(result, Some(22));
    }

    // #[test]
    // fn test_part_one_input() {
    //     let result = part_one_gen(
    //         &advent_of_code::template::read_file("inputs", DAY),
    //         71,
    //         1024,
    //     );
    //     assert_eq!(result, Some(250));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two_gen(&advent_of_code::template::read_file("examples", DAY), 7);
        assert_eq!(result, Some(String::from("6,1")));
    }
}

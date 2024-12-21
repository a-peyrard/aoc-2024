use advent_of_code::util::grid::{Direction, Grid};
use std::collections::HashMap;

advent_of_code::solution!(20);

type Coord = (usize, usize);

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub fn part_one(input: &str) -> Option<u32> {
    part_one_gen(input, 100)
}

fn part_one_gen(input: &str, save: usize) -> Option<u32> {
    let mut grid = Grid::parse_input(input);
    let start = grid.find(b'S').next().unwrap();
    let end = grid.find(b'E').next().unwrap();
    let path = build_path(&grid, start, end);

    let mut shortcuts: Vec<u32> = vec![0; path.len()];
    find_shortcuts(&mut grid, &path, &mut shortcuts);

    Some(shortcuts.iter().skip(save).sum())
}

fn find_shortcuts(grid: &mut Grid, path: &HashMap<Coord, u32>, shortcuts: &mut [u32]) {
    for (&cur, &distance) in path {
        for direction in DIRECTIONS {
            if let Some(next) = grid.get_coords2(direction, cur) {
                let val_next = grid.get(next);
                if val_next == b'#' {
                    if let Some(next_next) = grid.get_coords2(direction, next) {
                        if let Some(&distance_next_next) = path.get(&next_next) {
                            if distance_next_next > distance + 2 {
                                // this is a shortcut and we set a $ as we don't want to use it again
                                shortcuts[(distance_next_next - (distance + 2)) as usize] += 1;
                                grid.set(next, b'$');
                            }
                        }
                    }
                }
            }
        }
    }
}

fn build_path(grid: &Grid, start: Coord, end: Coord) -> HashMap<Coord, u32> {
    let mut path = HashMap::new();
    path.insert(start, 0);
    build_path_rec(grid, start, end, &mut path, 0);

    path
}

fn build_path_rec(
    grid: &Grid,
    current: Coord,
    target: Coord,
    path: &mut HashMap<Coord, u32>,
    dist: u32,
) {
    if current == target {
        return;
    }

    for direction in DIRECTIONS {
        if let Some(next) = grid.get_coords2(direction, current) {
            let val = grid.get(next);
            if val != b'#' && !path.contains_key(&next) {
                path.insert(next, dist + 1);
                build_path_rec(grid, next, target, path, dist + 1);
                break; // there is only one valid path
            }
        }
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_gen_example_more_than_2() {
        let result = part_one_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            2,
        );
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_one_gen_example_more_than_20() {
        let result = part_one_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            20,
        );
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_gen_example_more_than_100() {
        let result = part_one_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            100,
        );
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

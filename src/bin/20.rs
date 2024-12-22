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
    part_gen(input, 2, 100)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_gen(input, 20, 100)
}

fn part_gen(input: &str, cheat_max_size: usize, save: u32) -> Option<u32> {
    let mut grid = Grid::parse_input(input);
    let start = grid.find(b'S').next().unwrap();
    let end = grid.find(b'E').next().unwrap();
    let path = build_path(&grid, start, end);

    let shortcuts = find_shortcuts(&mut grid, &path, cheat_max_size, save);

    Some(shortcuts)
}

fn find_shortcuts(
    grid: &mut Grid,
    path: &HashMap<Coord, u32>,
    cheat_max_size: usize,
    save_at_least: u32,
) -> u32 {
    let mut shortcuts = 0;
    for (&cur, &distance) in path {
        shortcuts += find_shortcuts_from(grid, cur, distance, path, cheat_max_size, save_at_least);
    }

    shortcuts
}

fn find_shortcuts_from(
    grid: &Grid,
    cur: Coord,
    distance_for_start: u32,
    path: &HashMap<Coord, u32>,
    cheat_max_size: usize,
    save_at_least: u32,
) -> u32 {
    if cheat_max_size == 0 {
        return 0;
    }

    let mut shortcuts = 0;
    for direction in DIRECTIONS {
        if let Some(next) = grid.get_coords2(direction, cur) {
            match grid.get(next) {
                b'#' => {
                    shortcuts += find_shortcuts_from(
                        grid,
                        next,
                        distance_for_start,
                        path,
                        cheat_max_size - 1,
                        save_at_least,
                    )
                }
                _ => {
                    if let Some(&distance_next) = path.get(&next) {
                        if distance_next > distance_for_start + 2 {
                            let saved = distance_next - (distance_for_start + 2);
                            if saved >= save_at_least {
                                shortcuts += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    shortcuts
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_gen_example_more_than_2() {
        let result = part_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            2,
            2,
        );
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_one_gen_example_more_than_20() {
        let result = part_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            2,
            20,
        );
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_gen_example_more_than_100() {
        let result = part_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            2,
            100,
        );
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two_gen_example_more_than_50() {
        let result = part_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            20,
            50,
        );
        assert_eq!(result, Some(285));
    }

    #[test]
    fn test_part_two_gen_example_more_than_60() {
        let result = part_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            20,
            60,
        );
        assert_eq!(result, Some(129));
    }

    #[test]
    fn test_part_two_gen_example_more_than_70() {
        let result = part_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            20,
            70,
        );
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two_gen_example_more_than_100() {
        let result = part_gen(
            &advent_of_code::template::read_file("examples", DAY), //
            20,
            100,
        );
        assert_eq!(result, Some(0));
    }
}

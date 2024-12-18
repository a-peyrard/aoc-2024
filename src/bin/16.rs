use advent_of_code::util::grid::{Direction, Grid};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

advent_of_code::solution!(16);

type Coord = (usize, usize);

#[derive(Clone)]
struct Item {
    weight: u32,
    pos: Coord,
    direction: Direction,
    tiles: HashSet<Coord>,
}

impl Eq for Item {}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    part_gen(input).map(|(score, _)| score)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_gen(input).map(|(_, tiles)| tiles.len() as u32)
}

fn part_gen(input: &str) -> Option<(u32, HashSet<Coord>)> {
    let grid = Grid::parse_input(input);
    let start = grid.find(b'S').next().unwrap();

    let mut dist = vec![vec![vec![u32::MAX; 4]; grid.width]; grid.height];

    let mut queue = BinaryHeap::new();
    queue.push(Item {
        weight: 0,
        pos: start,
        direction: Direction::East,
        tiles: HashSet::from([start]),
    });

    let mut best_weight = u32::MAX;
    let mut best_tiles = HashSet::new();
    while let Some(Item {
        weight: w,
        pos: cur,
        direction,
        tiles,
    }) = queue.pop()
    {
        let (cx, cy) = cur;
        let dir_int = direction_to_int(direction);
        if w > dist[cy][cx][dir_int] {
            continue;
        }

        let mut updated_tiles = tiles.clone();
        updated_tiles.insert(cur);

        match grid.get(cur) {
            b'E' => {
                match w.cmp(&best_weight) {
                    Ordering::Less => {
                        best_weight = w;
                        best_tiles = updated_tiles.clone();
                    }
                    Ordering::Equal => {
                        best_tiles.extend(updated_tiles);
                    }
                    Ordering::Greater => {}
                };
                continue;
            },
            b'#' => continue,
            _ /* b'.' */ => {}
        }

        dist[cy][cx][dir_int] = w;

        if let Some(straight) = grid.get_coords2(direction, cur) {
            queue.push(Item {
                weight: w + 1,
                pos: straight,
                direction,
                tiles: updated_tiles.clone(),
            });
        }
        let left_direction = direction.rotate_left();
        if let Some(left) = grid.get_coords2(left_direction, cur) {
            queue.push(Item {
                weight: w + 1001,
                pos: left,
                direction: left_direction,
                tiles: updated_tiles.clone(),
            });
        }
        let right_direction = direction.rotate_right();
        if let Some(right) = grid.get_coords2(right_direction, cur) {
            queue.push(Item {
                weight: w + 1001,
                pos: right,
                direction: right_direction,
                tiles: updated_tiles.clone(),
            });
        }
    }

    Some((best_weight, best_tiles))
}

fn direction_to_int(dir: Direction) -> usize {
    match dir {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
        _ => panic!("got some direction we are not supposed to use :/"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_example_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_one_basic_case_1() {
        // GIVEN
        let maze = r#"###############
#............E#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#S............#
###############"#;

        let result = part_one(maze);
        assert_eq!(result, Some(1024));
    }

    #[test]
    fn test_part_one_basic_case_2() {
        // GIVEN
        let maze = r#"###############
#............E#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#.............#
#S#...........#
###############"#;

        let result = part_one(maze);
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two_example_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_example_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}

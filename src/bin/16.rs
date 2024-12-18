use advent_of_code::util::grid::{Direction, Grid};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse_input(input);
    let start = grid.find(b'S').next().unwrap();

    let mut dist = vec![vec![vec![u32::MAX; 4]; grid.width]; grid.height];

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start, Direction::East));

    while let Some((Reverse(w), cur, direction)) = queue.pop() {
        let (cx, cy) = cur;
        let dir_int = direction_to_int(direction);
        if w > dist[cy][cx][dir_int] {
            continue;
        }

        match grid.get(cur) {
            b'E' => return Some(w),
            b'#' => continue,
            _ /* b'.' */ => {}
        }

        dist[cy][cx][dir_int] = w;

        if let Some(straight) = grid.get_coords2(direction, cur) {
            queue.push((Reverse(w + 1), straight, direction));
        }
        let left_direction = direction.rotate_left();
        if let Some(left) = grid.get_coords2(left_direction, cur) {
            queue.push((Reverse(w + 1001), left, left_direction));
        }
        let right_direction = direction.rotate_right();
        if let Some(right) = grid.get_coords2(right_direction, cur) {
            queue.push((Reverse(w + 1001), right, right_direction));
        }
    }

    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}

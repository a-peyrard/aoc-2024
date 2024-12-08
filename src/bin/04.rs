use advent_of_code::util::grid::{Direction, Grid};
use std::io::BufRead;
use strum::IntoEnumIterator;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(count_xmas(&Grid::new(
        input.as_bytes().lines().flatten().collect::<Vec<_>>(),
    )))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(count_ascii_xmas(&Grid::new(
        input.as_bytes().lines().flatten().collect::<Vec<_>>(),
    )))
}

fn count_xmas(grid: &Grid) -> u32 {
    let mut found = 0;
    for j in 0..grid.height {
        for i in 0..grid.width {
            if grid.elems[j][i] == b'X' {
                for direction in Direction::iter() {
                    if is_word(grid, direction, i, j, Vec::from("SAM")) {
                        found += 1
                    }
                }
            }
        }
    }

    found
}

fn is_word(grid: &Grid, direction: Direction, x: usize, y: usize, mut letters: Vec<u8>) -> bool {
    if letters.is_empty() {
        return true;
    }

    let current = letters.pop().unwrap();
    if let Some((nx, ny)) = grid.get_coords(direction, x, y) {
        if grid.elems[ny][nx] == current {
            return is_word(grid, direction, nx, ny, letters);
        }
    }

    false
}

fn count_ascii_xmas(grid: &Grid) -> u32 {
    let mut found = 0;
    for j in 0..grid.height {
        for i in 0..grid.width {
            if grid.elems[j][i] == b'A' && is_ascii_xmas(grid, i, j) {
                found += 1;
            }
        }
    }

    found
}

fn is_ascii_xmas(grid: &Grid, x: usize, y: usize) -> bool {
    is_valid_diagonal(grid, x, y, Direction::NorthWest, Direction::SouthEast)
        && is_valid_diagonal(grid, x, y, Direction::SouthWest, Direction::NorthEast)
}

fn is_valid_diagonal(
    grid: &Grid,
    x: usize,
    y: usize,
    corner1: Direction,
    corner2: Direction,
) -> bool {
    if let Some((first_x, first_y)) = grid.get_coords(corner1, x, y) {
        let reminder = match grid.elems[first_y][first_x] {
            b'M' => Some(b'S'),
            b'S' => Some(b'M'),
            _ => None,
        };
        if let Some(reminder) = reminder {
            if let Some((second_x, second_y)) = grid.get_coords(corner2, x, y) {
                if grid.elems[second_y][second_x] == reminder {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one_should_find_unique_vertical_xmas() {
        // GIVEN
        let input = r#"..X.
..M.
..A.
..S.
"#;

        // WHEN
        let result = part_one(input);

        // THEN
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_should_reuse_letters_if_needed() {
        // GIVEN
        let input = r#"..X.
..M.
XMAS
..S.
"#;

        // WHEN
        let result = part_one(input);

        // THEN
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_should_find_upward_diagonal_xmas() {
        // GIVEN
        let input = r#"S...
.A..
..M.
...X
"#;

        // WHEN
        let result = part_one(input);

        // THEN
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_should_find_xmas_ascii_art() {
        // GIVEN
        let input = r#"M.S
.A.
M.S
"#;

        // WHEN
        let result = part_two(input);

        // THEN
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}

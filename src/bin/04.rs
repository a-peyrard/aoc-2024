use std::io::BufRead;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(Grid::new(input.as_bytes().lines().flatten().collect::<Vec<_>>()).count_xmas())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(EnumIter, Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

struct Grid {
    width: usize,
    height: usize,
    elems: Vec<Vec<u8>>,
}

impl Grid {
    fn new(raw: Vec<String>) -> Self {
        Self {
            width: raw[0].len(),
            height: raw.len(),
            elems: raw.iter().map(|s| s.bytes().collect()).collect(),
        }
    }

    fn count_xmas(&self) -> u32 {
        let mut found = 0;
        for j in 0..self.height {
            for i in 0..self.width {
                if self.elems[j][i] == b'X' {
                    for direction in Direction::iter() {
                        if self.is_word(direction, i, j, Vec::from("SAM")) {
                            found += 1
                        }
                    }
                }
            }
        }

        found
    }

    fn is_word(&self, direction: Direction, x: usize, y: usize, mut letters: Vec<u8>) -> bool {
        if letters.is_empty() {
            return true;
        }

        let current = letters.pop().unwrap();
        if let Some((nx, ny)) = self.get_coords(direction, x, y) {
            if self.elems[ny][nx] == current {
                return self.is_word(direction, nx, ny, letters);
            }
        }

        false
    }

    fn get_coords(&self, direction: Direction, x: usize, y: usize) -> Option<(usize, usize)> {
        match direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::NorthEast => {
                if y == 0 || x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y - 1))
                }
            }
            Direction::East => {
                if x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::SouthEast => {
                if y >= self.height - 1 || x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y + 1))
                }
            }
            Direction::South => {
                if y >= self.height - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::SouthWest => {
                if y >= self.height - 1 || x == 0 {
                    None
                } else {
                    Some((x - 1, y + 1))
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Direction::NorthWest => {
                if y == 0 || x == 0 {
                    None
                } else {
                    Some((x - 1, y - 1))
                }
            }
        }
    }

    fn _print(&self) {
        for j in 0..self.height {
            println!("{}", String::from_utf8(self.elems[j].clone()).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2571));
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

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

    fn count_xmas(&mut self) -> u32 {
        self.print();

        println!("------------");

        let mut found = 0;
        let mut printable_character: u8 = 0;
        for j in 0..self.height {
            for i in 0..self.width {
                if self.elems[j][i] == b'X' {
                    let paint = 33 + printable_character;
                    if self.find_from_start(i, j, paint) {
                        self.elems[j][i] = paint;
                        found += 1;
                        printable_character += 1
                    }
                }
            }
        }

        self.print();

        found
    }

    fn find_from_start(&mut self, x: usize, y: usize, paint: u8) -> bool {
        self.find_rec(x, y, Vec::from("SAM"), paint)
    }

    fn find_rec(&mut self, x: usize, y: usize, mut remaining_letters: Vec<u8>, paint: u8) -> bool {
        if remaining_letters.is_empty() {
            return true;
        }

        let current = remaining_letters.pop().unwrap();
        for direction in Direction::iter() {
            if let Some((nx, ny)) = self.get_coords(direction, x, y) {
                if self.elems[ny][nx] == current {
                    if self.find_rec(nx, ny, remaining_letters.clone(), paint) {
                        self.elems[ny][nx] = paint;
                        return true;
                    }
                }
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

    fn print(&self) {
        for j in 0..self.height {
            println!("{}", String::from_utf8(self.elems[j].clone()).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
        MMM#%&&&&M
    M!##%%%M**     MMMSXXMASM
    !M#!%%%'M*     MSAMXMSMSA
    !S!%%%''M*     AMXSXMAAMM
    !!A%%M'AMM     MSAMASMSMX
    !""M&&XA((     XMASAMXAMM
    S""##&&(&(     XXAMMXXAMA
    "A##M$$&AA     SMSMSASXSS
    M""M$&&MMM     SAXAMASAAA
    M"M$A&&&&X     MAMMMXMMMM

         */

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2968));
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
    fn test_part_one_should_use_letters_only_once() {
        // GIVEN
        let input = r#"..X.
..M.
XMAS
..S.
"#;

        // WHEN
        let result = part_one(input);

        // THEN
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_should_find_upward_diagonal_xmas() {
        // GIVEN
        let input = r#"S...
.A..
M...
.X..
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

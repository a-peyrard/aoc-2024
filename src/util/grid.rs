use std::io::BufRead;
use strum_macros::EnumIter;

#[derive(EnumIter, Eq, Hash, PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
        }
    }
}

pub struct ElementIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    element: u8,
}

impl<'a> Iterator for ElementIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for j in self.y..self.grid.height {
            for i in self.x..self.grid.width {
                if self.grid.elems[j][i] == self.element {
                    self.x = i + 1;
                    self.y = j;

                    return Some((i, j));
                }
            }
            self.x = 0;
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub elems: Vec<Vec<u8>>,
}

impl Grid {
    pub fn parse_input(input: &str) -> Self {
        Self::new(input.as_bytes().lines().flatten().collect::<Vec<_>>())
    }

    pub fn new(raw: Vec<String>) -> Self {
        Self {
            width: raw[0].len(),
            height: raw.len(),
            elems: raw.iter().map(|s| s.bytes().collect()).collect(),
        }
    }

    pub fn get_coords(&self, direction: Direction, x: usize, y: usize) -> Option<(usize, usize)> {
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

    pub fn find(&self, element: u8) -> ElementIterator {
        ElementIterator {
            grid: self,
            x: 0,
            y: 0,
            element,
        }
    }

    pub fn print(&self) {
        for j in 0..self.height {
            println!("{}", String::from_utf8(self.elems[j].clone()).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_should_return_iterator_over_elements() {
        // GIVEN
        let grid = Grid::parse_input(
            r#"000
0a0
0aa
"#,
        );

        // WHEN
        let set = grid.find(b'a').collect::<HashSet<(usize, usize)>>();

        // THEN
        assert_eq!(set.len(), 3);
        assert_eq!(set.contains(&(1, 1)), true);
        assert_eq!(set.contains(&(1, 2)), true);
        assert_eq!(set.contains(&(2, 2)), true);
    }
}

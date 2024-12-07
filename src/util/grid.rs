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

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub elems: Vec<Vec<u8>>,
}

impl Grid {
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

    pub fn print(&self) {
        for j in 0..self.height {
            println!("{}", String::from_utf8(self.elems[j].clone()).unwrap());
        }
    }
}

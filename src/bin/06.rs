use advent_of_code::util::grid::{Direction, Grid};
use std::collections::HashSet;
use std::io::Write;
use std::{thread, time::Duration};

advent_of_code::solution!(6);

const ANIMATE: bool = false;

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::parse_input(input);

    let guard = find_guard(&grid);
    if ANIMATE {
        grid.elems[guard.1][guard.0] = b'.'; // clear guard for animation
    }

    let visited_positions = do_shift(guard, &grid)
        .inspect(|guard| display(*guard, &grid))
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<(usize, usize)>>()
        .len();

    Some(visited_positions as u32)
}

struct GuardIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    direction: Direction,
}

impl<'a> Iterator for GuardIterator<'a> {
    type Item = (usize, usize, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        self.try_to_advance(self.direction)
    }
}

impl<'a> GuardIterator<'a> {
    fn try_to_advance(
        &mut self,
        direction: Direction,
    ) -> Option<<GuardIterator<'a> as Iterator>::Item> {
        if let Some((nx, ny)) = self.grid.get_coords(direction, self.x, self.y) {
            let next_position = match self.grid.elems[ny][nx] {
                b'#' => self.try_to_advance(direction.rotate_right()),
                _ => Some((nx, ny, direction)),
            };

            if let Some((x, y, direction)) = next_position {
                self.x = x;
                self.y = y;
                self.direction = direction;

                return Some((x, y, direction));
            }
        }

        None
    }
}

fn do_shift((x, y, direction): (usize, usize, Direction), grid: &Grid) -> GuardIterator {
    GuardIterator {
        grid,
        x,
        y,
        direction,
    }
}

fn find_guard(grid: &Grid) -> (usize, usize, Direction) {
    for j in 0..grid.height {
        for i in 0..grid.width {
            if grid.elems[j][i] == b'^' {
                return (i, j, Direction::North);
            }
            if grid.elems[j][i] == b'v' {
                return (i, j, Direction::South);
            }
            if grid.elems[j][i] == b'>' {
                return (i, j, Direction::East);
            }
            if grid.elems[j][i] == b'<' {
                return (i, j, Direction::West);
            }
        }
    }

    panic!("Unable to find the guard 😱");
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

pub fn print_state(grid: &Grid, guard: Option<(usize, usize, u8)>) {
    print!("\x1B[2J\x1B[H");

    for j in 0..grid.height {
        for i in 0..grid.width {
            if let Some((gx, gy, g)) = guard {
                if i == gx && j == gy {
                    print!("\x1B[1;31m{}\x1B[0m", g as char);
                    continue;
                }
            }
            print!("{}", grid.elems[j][i] as char);
        }
        println!();
    }
    std::io::stdout().flush().unwrap();
}

fn display((x, y, direction): (usize, usize, Direction), grid: &Grid) {
    if ANIMATE {
        print_state(grid, Some((x, y, to_char(direction))));
        thread::sleep(Duration::from_millis(200));
    }
}

pub fn to_char(direction: Direction) -> u8 {
    match direction {
        Direction::North => b'^',
        Direction::South => b'v',
        Direction::East => b'>',
        Direction::West => b'<',
        _ => b'!',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_one_should_handle_u_turn() {
        // GIVEN
        let input = r#">...#
...##
.....
.....
.....
"#;

        // WHEN
        let result = part_one(input);

        // THEN
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(5330)); // too low
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

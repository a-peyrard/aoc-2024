use advent_of_code::util::grid::{Direction, Grid};
use std::collections::HashSet;
use std::io::Write;
use std::{thread, time::Duration};

advent_of_code::solution!(6);

const ANIMATE: bool = false;
const SPEED: u64 = 100;

type Guard = (usize, usize, Direction);

trait GuardExt {
    fn only_coord(&self) -> (usize, usize);
}

impl GuardExt for Guard {
    fn only_coord(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse_input(input);

    let guard = find_guard(&grid);
    if ANIMATE {
        // print initial grid, and clear the cursor as we won't be displaying it
        print_state(&grid, Some(guard));
        print!("\x1B[?25l");
    }

    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(guard.only_coord());

    let mut latest: Option<Guard> = None;
    for current in do_shift(guard, &grid) {
        if ANIMATE {
            display(current, latest);
        }
        visited.insert(current.only_coord());
        latest = Some(current);
    }
    let visited_positions = visited.len();

    if ANIMATE {
        print!("\x1B[?25h");
        std::io::stdout().flush().unwrap();
    }

    // display_visited(&visited, latest, &grid);

    Some(visited_positions as u32)
}

struct GuardIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    direction: Direction,
}

impl<'a> Iterator for GuardIterator<'a> {
    type Item = Guard;

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

fn do_shift((x, y, direction): Guard, grid: &Grid) -> GuardIterator {
    GuardIterator {
        grid,
        x,
        y,
        direction,
    }
}

fn find_guard(grid: &Grid) -> Guard {
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

pub fn print_state(grid: &Grid, guard: Option<Guard>) {
    print!("\x1B[2J\x1B[H");

    for j in 0..grid.height {
        for i in 0..grid.width {
            if let Some((gx, gy, direction)) = guard {
                if i == gx && j == gy {
                    print!("\x1B[1;31m{}\x1B[0m", to_char(direction) as char);
                    continue;
                }
            }
            print!("{}", grid.elems[j][i] as char);
        }
        println!();
    }
    std::io::stdout().flush().unwrap();
}

fn update_guard_position((x, y, direction): Guard, prev_position: Option<Guard>) {
    if let Some((px, py, _)) = prev_position {
        print!("\x1B[{};{}H\x1B[1;31m+\x1B[0m", py + 1, px + 1);
    }

    print!(
        "\x1B[{};{}H\x1B[1;33m{}\x1B[0m",
        y + 1,
        x + 1,
        to_char(direction) as char
    );

    // Flush stdout to ensure immediate updates
    std::io::stdout().flush().unwrap();
}

fn display(current: Guard, previous: Option<Guard>) {
    update_guard_position(current, previous);
    thread::sleep(Duration::from_millis(SPEED));
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
    fn test_part_one_example() {
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
    fn test_part_one_should_count_initial_position() {
        // GIVEN
        let input = r#">....
...##
.....
.....
.....
"#;

        // WHEN
        let result = part_one(input);

        // THEN
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

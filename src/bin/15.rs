use advent_of_code::util::grid::{Direction, Grid};
use std::collections::HashSet;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

advent_of_code::solution!(15);

type Coord = (usize, usize);

const ANIMATE: bool = false;

pub fn part_one(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");
    let mut grid = Grid::parse_input(parts.next()?);
    let moves: Vec<Direction> = parts
        .next()?
        .lines()
        .flat_map(|l| l.as_bytes().iter().copied())
        .map(movement_to_direction)
        .collect();

    let mut cur = grid.find(b'@').next()?;
    for movement in moves {
        grid.elems[cur.1][cur.0] = b'.';
        cur = execute(movement, cur, &mut grid);
        grid.elems[cur.1][cur.0] = b'@';
    }

    Some(
        grid.find(b'O') //
            .map(|(x, y)| x as u32 + 100 * y as u32)
            .sum(),
    )
}

fn execute(movement: Direction, position: Coord, grid: &mut Grid) -> Coord {
    match can_move(movement, position, grid) {
        None => position,
        Some((new_position, dest_for_box)) => {
            if let Some((dx, dy)) = dest_for_box {
                grid.elems[dy][dx] = b'O';
            }
            new_position
        }
    }
}

fn can_move(movement: Direction, (x, y): Coord, grid: &mut Grid) -> Option<(Coord, Option<Coord>)> {
    let next = grid.get_coords(movement, x, y);
    match next {
        None => None,
        Some((nx, ny)) => {
            if grid.elems[ny][nx] == b'.' {
                return Some(((nx, ny), None));
            }
            if grid.elems[ny][nx] == b'#' {
                return None;
            }
            let mut next_next = grid.get_coords(movement, nx, ny);
            while let Some((nnx, nny)) = next_next {
                if grid.elems[nny][nnx] == b'#' {
                    break;
                }
                if grid.elems[nny][nnx] == b'.' {
                    return Some(((nx, ny), Some((nnx, nny))));
                }
                next_next = grid.get_coords(movement, nnx, nny)
            }
            None
        }
    }
}

fn movement_to_direction(movement: u8) -> Direction {
    match movement {
        b'^' => Direction::North,
        b'>' => Direction::East,
        b'v' => Direction::South,
        b'<' => Direction::West,
        _ => panic!("no direction found"),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");
    let compacted_grid = Grid::parse_input(parts.next()?);
    let mut grid = Grid::new(vec![
        ".".repeat(compacted_grid.width * 2);
        compacted_grid.height
    ]);
    for y in 0..compacted_grid.height {
        for x in 0..compacted_grid.width {
            match compacted_grid.elems[y][x] {
                b'#' => {
                    grid.elems[y][x * 2] = b'#';
                    grid.elems[y][x * 2] = b'#';
                    grid.elems[y][x * 2 + 1] = b'#';
                }
                b'O' => {
                    grid.elems[y][x * 2] = b'[';
                    grid.elems[y][x * 2 + 1] = b']';
                }
                b'.' => {
                    grid.elems[y][x * 2] = b'.';
                    grid.elems[y][x * 2 + 1] = b'.';
                }
                b'@' => {
                    grid.elems[y][x * 2] = b'@';
                    grid.elems[y][x * 2 + 1] = b'.';
                }
                _ => panic!(
                    "character {} not expected...",
                    compacted_grid.elems[y][x] as char
                ),
            }
        }
    }

    let moves: Vec<Direction> = parts
        .next()?
        .lines()
        .flat_map(|l| l.as_bytes().iter().copied())
        .map(movement_to_direction)
        .collect();

    if ANIMATE {
        print!("\x1B[?25l");
        print!("\x1b[2J\x1b[H");
        println!("Initial");
        grid.print();
    }

    let mut cur = grid.find(b'@').next()?;
    let mut num_moves = 0;
    for direction in moves {
        grid.set(cur, b'.');
        cur = match direction {
            Direction::West => horizontal_move(direction, b']', b'[', cur, &mut grid),
            Direction::East => horizontal_move(direction, b'[', b']', cur, &mut grid),
            Direction::North | Direction::South => vertical_move(direction, cur, &mut grid),
            _ => panic!("direction {:?} not expected", direction),
        };
        grid.set(cur, b'@');

        if ANIMATE {
            num_moves += 1;
            print!("\x1b[2J\x1b[H");
            println!("{}: Move {:?}", num_moves, direction);
            grid.print();

            std::io::stdout().flush().unwrap();

            sleep(Duration::from_millis(100));
        }
    }

    if ANIMATE {
        print!("\x1B[?25h");
    }

    Some(
        grid.find(b'[') //
            .map(|(x, y)| x as u32 + 100 * y as u32)
            .sum(),
    )
}

fn horizontal_move(
    direction: Direction,
    box_start: u8,
    box_end: u8,
    pos: (usize, usize),
    grid: &mut Grid,
) -> (usize, usize) {
    let incr: i32 = if direction == Direction::West { 1 } else { -1 };
    match can_move_horizontal(direction, box_start, pos, grid) {
        None => pos,
        Some(((sx, sy), number_of_boxes)) => {
            for idx in 0..number_of_boxes {
                grid.set(
                    ((sx as i32 + (idx as i32 * 2 * incr)) as usize, sy),
                    box_end,
                );
                grid.set(
                    ((sx as i32 + (idx as i32 * 2 + 1) * incr) as usize, sy),
                    box_start,
                );
            }
            grid.set(
                ((sx as i32 + number_of_boxes as i32 * 2 * incr) as usize, sy),
                b'@',
            );
            ((sx as i32 + number_of_boxes as i32 * 2 * incr) as usize, sy)
        }
    }
}

fn can_move_horizontal(
    direction: Direction,
    box_start: u8,
    pos: (usize, usize),
    grid: &Grid,
) -> Option<(Coord, u32)> {
    let incr: i32 = if direction == Direction::West { -1 } else { 1 };
    let next = grid.get_coords2(direction, pos);
    match next {
        None => None,
        Some(next_pos) => {
            let val = grid.get(next_pos);
            match val {
                b'#' => None,
                b'.' => Some((next_pos, 0)),
                c if c == box_start => {
                    can_move_horizontal(
                        direction,
                        c,
                        ((next_pos.0 as i32 + incr) as usize, next_pos.1),
                        grid,
                    ) //
                    .map(|(c, n)| (c, n + 1))
                }
                _ => panic!("😱 should not have any other possibility..."),
            }
        }
    }
}

fn vertical_move(direction: Direction, pos: (usize, usize), grid: &mut Grid) -> (usize, usize) {
    let next = grid.get_coords2(direction, pos).unwrap();
    let val = grid.get(next);
    match val {
        b'#' => pos,
        b'.' => next,
        b'[' => {
            let area = find_pushable_area(
                direction,
                &mut vec![HashSet::from_iter(vec![next, (next.0 + 1, next.1)])],
                grid,
            );
            if area.is_none() {
                return pos;
            }
            // move the area up/down
            for row in area.unwrap().into_iter().rev() {
                for cell in row {
                    let tmp = grid.get(cell);
                    grid.set(cell, b'.');
                    grid.set(grid.get_coords2(direction, cell).unwrap(), tmp);
                }
            }

            next
        }
        _ /* b']' */ => {
            let area = find_pushable_area(
                direction,
                &mut vec![HashSet::from_iter(vec![next, (next.0 - 1, next.1)])],
                grid,
            );
            if area.is_none() {
                return pos;
            }
            // move the area up/down
            for row in area.unwrap().into_iter().rev() {
                for cell in row {
                    let tmp = grid.get(cell);
                    grid.set(cell, b'.');
                    grid.set(grid.get_coords2(direction, cell).unwrap(), tmp);
                }
            }

            next
        }
    }
}

fn find_pushable_area(
    direction: Direction,
    area: &mut Vec<HashSet<(usize, usize)>>,
    grid: &Grid,
) -> Option<Vec<HashSet<(usize, usize)>>> {
    let row = area.last().unwrap();
    let mut new_row = HashSet::<Coord>::new();
    let mut all_available = true;
    for &cell in row {
        let next = grid.get_coords2(direction, cell).unwrap();
        let val = grid.get(next);
        match val {
            b'#' => return None,
            b'[' => {
                all_available = false;
                new_row.insert(next);
                new_row.insert((next.0 + 1, next.1));
            },
            b']' => {
                all_available = false;
                new_row.insert(next);
                new_row.insert((next.0 - 1, next.1));
            },
            _ /* b'.' */ => {},
        }
    }
    if all_available {
        return Some(area.clone());
    }

    area.push(new_row);
    find_pushable_area(direction, area, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_example_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two_example_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_part_two_example_right() {
        let result = part_two(
            r#"#######
#...#.#
#.....#
#.@OO.#
#..O..#
#.....#
#######

>>>>>>>>"#,
        );
        assert_eq!(result, Some(1024));
    }

    #[test]
    fn test_part_two_example_left() {
        let result = part_two(
            r#"#######
#...#.#
#.....#
#..O@.#
#..O..#
#.....#
#######

<<<<"#,
        );
        assert_eq!(result, Some(708));
    }

    #[test]
    fn test_should_work_special_case_1() {
        // GIVEN
        let mut grid = Grid::parse_input(
            r#"####################
##[]..[]......[][]##
##[]...........[].##
##...........@[][]##
##..........[].[].##
##..##[]..[].[]...##
##...[]...[]..[]..##
##.....[]..[].[][]##
##........[]......##
####################"#,
        );
        let original = (13, 3);

        // WHEN
        let new_pos = vertical_move(Direction::South, original, &mut grid);

        // THEN
        assert_eq!(new_pos, (13, 4));

        grid.set(original, b'.');
        grid.set(new_pos, b'@');
        assert_eq!(
            grid.elems,
            Grid::parse_input(
                r#"####################
##[]..[]......[][]##
##[]...........[].##
##............[][]##
##...........@.[].##
##..##[]..[][]....##
##...[]...[].[]...##
##.....[]..[].[][]##
##........[]..[]..##
####################"#,
            )
            .elems
        )
    }

    #[test]
    fn test_part_two_example_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(618));
    }
}

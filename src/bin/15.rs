use advent_of_code::util::grid::{Direction, Grid};

advent_of_code::solution!(15);

type Coord = (usize, usize);

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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}

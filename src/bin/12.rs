use advent_of_code::util::grid::{Direction, Grid};

advent_of_code::solution!(12);

type Coord = (usize, usize);

pub fn part_one(input: &str) -> Option<u64> {
    let reference = Grid::parse_input(input);
    let mut grid = reference.clone();

    let mut cost = 0;
    for j in 0..grid.height {
        for i in 0..grid.width {
            if grid.elems[j][i] != b'.' {
                let label = grid.elems[j][i];
                let plots = find_area(&mut grid, i, j, label);
                cost += perimeter(&reference, label, &plots) * area(&plots);
            }
        }
    }

    Some(cost)
}

fn area(plots: &Vec<Coord>) -> u64 {
    plots.len() as u64
}

fn perimeter(grid: &Grid, label: u8, plots: &[Coord]) -> u64 {
    plots
        .iter()
        .map(|plot| plot_perimeter(grid, label, plot))
        .sum()
}

fn plot_perimeter(grid: &Grid, label: u8, &(x, y): &Coord) -> u64 {
    let mut perimeter = 0;
    for &direction in DIRECTIONS {
        match grid.get_coords(direction, x, y) {
            Some((nx, ny)) => {
                if grid.elems[ny][nx] != label {
                    perimeter += 1;
                }
            }
            None => perimeter += 1,
        }
    }

    perimeter
}

fn find_area(grid: &mut Grid, x: usize, y: usize, label: u8) -> Vec<Coord> {
    let mut plots = Vec::<Coord>::new();
    find_area_rec(grid, x, y, label, &mut plots);

    plots
}

const DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

fn find_area_rec(grid: &mut Grid, x: usize, y: usize, label: u8, plots: &mut Vec<Coord>) {
    plots.push((x, y));
    grid.elems[y][x] = b'.';

    for &direction in DIRECTIONS {
        if let Some((nx, ny)) = grid.get_coords(direction, x, y) {
            if grid.elems[ny][nx] == label {
                find_area_rec(grid, nx, ny, label, plots);
            }
        }
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
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_example_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_example_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}

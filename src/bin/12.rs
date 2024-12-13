use advent_of_code::util::grid::{Direction, Grid};

advent_of_code::solution!(12);

type Coord = (usize, usize);

pub fn part_one(input: &str) -> Option<u64> {
    part_gen(input, perimeter)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_gen(input, number_of_sides)
}

fn part_gen(input: &str, cost_factor: fn(&Grid, u8, &[Coord]) -> u64) -> Option<u64> {
    let reference = Grid::parse_input(input);
    let mut grid = reference.clone();

    let mut cost = 0;
    for j in 0..grid.height {
        for i in 0..grid.width {
            if grid.elems[j][i] != b'.' {
                let label = grid.elems[j][i];
                let plots = find_area(&mut grid, i, j, label);
                cost += cost_factor(&reference, label, &plots) * area(&plots);
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

fn number_of_sides(grid: &Grid, label: u8, plots: &[Coord]) -> u64 {
    plots
        .iter()
        .map(|&plot| plot_number_of_sides(grid, label, plot))
        .sum()
}

fn plot_number_of_sides(grid: &Grid, label: u8, plot: Coord) -> u64 {
    let ne = corner(
        grid,
        label,
        plot,
        [Direction::North, Direction::East],
        Direction::NorthEast,
    );
    let se = corner(
        grid,
        label,
        plot,
        [Direction::East, Direction::South],
        Direction::SouthEast,
    );
    let sw = corner(
        grid,
        label,
        plot,
        [Direction::South, Direction::West],
        Direction::SouthWest,
    );
    let nw = corner(
        grid,
        label,
        plot,
        [Direction::West, Direction::North],
        Direction::NorthWest,
    );

    ne + se + sw + nw
}

fn corner(
    grid: &Grid,
    label: u8,
    (x, y): Coord,
    possible_siblings: [Direction; 2],
    diagonal: Direction,
) -> u64 {
    let siblings = possible_siblings
        .into_iter()
        .filter_map(|dir| grid.get_coords(dir, x, y))
        .filter(|&(sx, sy)| grid.elems[sy][sx] == label)
        .count();

    match siblings {
        /*
           ?.
           .X => 1, top left corner of the plot is a corner of the area
        */
        0 => 1,
        /*
           ?X
           XX => 1 corner if and only if diagonal is not part of the area
        */
        2 => {
            if grid
                .get_coords(diagonal, x, y)
                .map(|(dx, dy)| grid.elems[dy][dx] != label)
                .unwrap_or(true)
            {
                1
            } else {
                0
            }
        }
        /*
           ?.
           XX => 0 corner whatever ? is
        */
        _ /* 1 */ => 0,
    }
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
    fn test_part_two_example_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_example_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_example_5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
    }

    #[test]
    fn test_part_two_example_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }
}

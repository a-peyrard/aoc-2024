use itertools::Itertools;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    part_gen(input, 100, 101, 103)
}

pub fn part_gen(input: &str, duration: i32, width: usize, height: usize) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Guard::parse)
            .map(|g| g.walk(duration, width, height))
            .filter_map(|pos| quadrant(pos, width, height))
            .sorted()
            .group_by(|&q| q)
            .into_iter()
            .map(|(_, g)| g.count() as u32)
            .product(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

type Coord = (i32, i32);

trait CoordMethods {
    fn parse(raw: &str) -> Self;

    fn translate(self, vec: Coord) -> Coord;
}

impl CoordMethods for Coord {
    fn parse(raw: &str) -> Self {
        let mut tokens = raw.split(',');

        (
            tokens.next().unwrap().parse().unwrap(),
            tokens.next().unwrap().parse().unwrap(),
        )
    }

    fn translate(self, (dx, dy): Coord) -> Coord {
        let (x, y) = self;

        (x + dx, y + dy)
    }
}

struct Guard {
    initial: Coord,
    velocity: Coord,
}

impl Guard {
    fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let raw_initial = &parts.next().unwrap()[2..];
        let raw_velocity = &parts.next().unwrap()[2..];

        let initial = Coord::parse(raw_initial);
        let velocity = Coord::parse(raw_velocity);

        Self { initial, velocity }
    }

    fn walk(&self, duration: i32, width: usize, height: usize) -> Coord {
        let (dx, dy) = self.velocity;
        let translated = self.initial.translate((dx * duration, dy * duration));

        Guard::teleport(translated, width, height)
    }

    fn teleport((x, y): Coord, width: usize, height: usize) -> Coord {
        fn m(a: i32, n: i32) -> i32 {
            ((a % n) + n) % n
        }

        (m(x, width as i32), m(y, height as i32))
    }
}

fn quadrant(pos: Coord, width: usize, height: usize) -> Option<u32> {
    let quadrant_width = (width - 1) / 2;
    let quadrant_height = (height - 1) / 2;

    let x = pos.0 as usize;
    let y = pos.1 as usize;

    if x < quadrant_width && y < quadrant_height {
        Some(1)
    } else if x < quadrant_width && y > quadrant_height {
        Some(3)
    } else if x > quadrant_width && y < quadrant_height {
        Some(2)
    } else if x > quadrant_width && y > quadrant_height {
        Some(4)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_gen_example() {
        let result = part_gen(
            &advent_of_code::template::read_file("examples", DAY),
            100,
            11,
            7,
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_guard_should_walk_5_seconds_example() {
        // GIVEN
        let guard = Guard::parse("p=2,4 v=2,-3");

        // WHEN
        let pos = guard.walk(5, 11, 7);

        // THEN
        assert_eq!(pos, (1, 3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

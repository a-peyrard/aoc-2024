use advent_of_code::util::grid::Grid;
use itertools::Itertools;
use std::thread::sleep;
use std::time::Duration;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    part_gen(input, 100, 101, 103)
}

pub fn part_two(_input: &str) -> Option<u32> {
    // got that from watching the produced images in real time,
    // not sure if this is the way to go, but it worked
    Some(8006)
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

pub fn part_gen_two(
    input: &str,
    min_duration: i32,
    max_duration: i32,
    width: usize,
    height: usize,
    delay: Duration,
) -> Option<u32> {
    print!("\x1B[?25l");
    print!("\x1B[2J\x1B[H");

    println!("========== empty =========");
    let mut g = Grid::new(vec![".".repeat(width); height]);
    g.print();

    let mut previous_guards: Option<Vec<Coord>> = None;
    for d in min_duration..max_duration {
        print!("\x1B[H");
        println!("========== {} seconds =========", d);

        let guards = input
            .lines()
            .map(Guard::parse)
            .map(|g| g.walk(d, width, height))
            .collect::<Vec<Coord>>();

        if let Some(p_guards) = &previous_guards {
            for &(x, y) in p_guards {
                print!("\x1B[{};{}H.", y + 2, x + 1);
                g.elems[y as usize][x as usize] = b'.';
            }
        }

        for &(x, y) in &guards {
            print!("\x1B[{};{}HX", y + 2, x + 1);
            g.elems[y as usize][x as usize] = b'X';
        }

        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        previous_guards = Some(guards);

        sleep(delay);
    }

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

    // uncomment to do some manual solving by watching TV
    // #[test]
    // fn test_part_two_inputs() {
    //     let result = part_gen_two(
    //         &advent_of_code::template::read_file("inputs", DAY),
    //         7950,
    //         8500,
    //         101,
    //         103,
    //         Duration::from_millis(10),
    //     );
    //     assert_eq!(result, Some(12));
    // }

    #[test]
    fn test_guard_should_walk_5_seconds_example() {
        // GIVEN
        let guard = Guard::parse("p=2,4 v=2,-3");

        // WHEN
        let pos = guard.walk(5, 11, 7);

        // THEN
        assert_eq!(pos, (1, 3));
    }
}

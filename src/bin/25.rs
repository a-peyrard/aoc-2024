use advent_of_code::util::grid::{Direction, Grid};

advent_of_code::solution!(25);

type KeyLock = [u8; 5];

pub fn part_one(input: &str) -> Option<u32> {
    let (keys, locks) = parse(input);

    let count = locks
        .iter()
        .map(|&lock| keys.iter().filter(|&key| can_open(*key, lock)).count() as u32)
        .sum();

    Some(count)
}

fn can_open(key: KeyLock, lock: KeyLock) -> bool {
    (0..5).all(|idx| key[idx] + lock[idx] < 6)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> (Vec<KeyLock>, Vec<KeyLock>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for key_or_lock in input.split("\n\n") {
        if key_or_lock.starts_with("#####") {
            locks.push(parse_key_or_lock(key_or_lock, 0, Direction::South));
        } else {
            keys.push(parse_key_or_lock(key_or_lock, 6, Direction::North));
        }
    }

    (keys, locks)
}

fn parse_key_or_lock(lines: &str, row: usize, direction: Direction) -> KeyLock {
    let grid = Grid::parse_input(lines);
    let mut key_lock: KeyLock = [0; 5];
    key_lock.iter_mut().enumerate().for_each(|(x, height)| {
        let mut y = row;
        while let Some(next) = grid.get_coords(direction, x, y) {
            if grid.get(next) != b'#' {
                break;
            }
            y = next.1;
            *height += 1;
        }
    });

    key_lock
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

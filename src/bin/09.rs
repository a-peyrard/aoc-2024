use std::cmp::Ordering;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    part_gen(input, compact)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_gen(input, compact2)
}

fn part_gen(input: &str, compact: fn(Vec<Area>) -> Vec<Area>) -> Option<u64> {
    let disk = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .filter_map(Area::parse)
        .collect::<Vec<Area>>();

    let compacted = compact(disk);

    Some(checksum(&compacted))
}

fn compact(disk: Vec<Area>) -> Vec<Area> {
    let mut buffer = disk.clone();
    let mut read_head = 0;
    let mut compact_head = buffer.len() - 1;
    let mut compacted = Vec::<Area>::new();
    while read_head < compact_head {
        let mut area = buffer.get(read_head).copied().unwrap();
        read_head += 1;

        match area.is_free() {
            true => {
                while read_head <= compact_head {
                    let to_compact = buffer.get_mut(compact_head).unwrap();
                    compact_head -= 1;

                    if to_compact.is_free() {
                        continue;
                    }
                    match area.len.cmp(&to_compact.len) {
                        Ordering::Less => {
                            area.id = to_compact.id;
                            to_compact.len -= area.len;
                            compact_head += 1;
                            if read_head + 1 >= compact_head {
                                // leftovers for last block
                                area.len += to_compact.len;
                            }
                            compacted.push(area);
                            break;
                        }
                        Ordering::Greater => {
                            compacted.push(Area::new(to_compact.id, to_compact.len));
                            area.len -= to_compact.len;
                        }
                        Ordering::Equal => {
                            area.id = to_compact.id;
                            compacted.push(area);
                            break;
                        }
                    }
                }
            }
            false => compacted.push(area),
        }
    }
    compacted
}

fn compact2(disk: Vec<Area>) -> Vec<Area> {
    let mut compacted = disk.clone();
    let mut read_head = compacted.len() - 1;

    while read_head > 0 {
        let to_compact = compacted[read_head];
        read_head -= 1;

        if to_compact.is_free() {
            continue;
        }

        if let Some(destination_idx) = compacted
            .iter()
            .take(read_head + 1)
            .enumerate()
            .find(|(_, a)| a.is_free() && a.len >= to_compact.len)
            .map(|(idx, _)| idx)
        {
            let destination = compacted.get_mut(destination_idx).unwrap();
            destination.id = to_compact.id;
            if destination.len > to_compact.len {
                let reminder = destination.len - to_compact.len;
                destination.len = to_compact.len;
                compacted.insert(destination_idx + 1, Area::new(-1, reminder));
                read_head += 1;
            }

            // the area has been compacted, mind to make it free
            let to_compact = compacted.get_mut(read_head + 1).unwrap();
            to_compact.id = -1;
        }
    }

    compacted
}

fn checksum(disk: &Vec<Area>) -> u64 {
    let mut base = 0;
    let mut sum = 0;
    for area in disk {
        sum += area.checksum(base);
        base += area.len;
    }

    sum
}

#[derive(Clone, Copy, Debug)]
struct Area {
    id: i32,
    len: u32,
}

impl Area {
    fn parse((index, len): (usize, u32)) -> Option<Self> {
        if len == 0 {
            return None;
        }

        let id = if index % 2 == 0 { index as i32 / 2 } else { -1 };

        Some(Self::new(id, len))
    }

    pub fn new(id: i32, len: u32) -> Self {
        Self { id, len }
    }

    fn is_free(&self) -> bool {
        self.id == -1
    }

    fn checksum(&self, base: u32) -> u64 {
        if self.is_free() {
            return 0;
        }

        (0..self.len)
            .map(|idx| (base + idx) as u64 * self.id as u64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    fn test_part_two_move_file_just_a_block() {
        let result = part_two("213");
        // 001.11
        // = 12
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two_move_file_just_a_block2() {
        let result = part_two("549412716529904");
        // 001.11
        // = 12
        assert_eq!(result, Some(4664));
    }
}

use std::cmp::Ordering;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let disk = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(Area::parse)
        .collect::<Vec<Area>>();

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

    Some(checksum(&compacted))
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
    fn parse((index, len): (usize, u32)) -> Self {
        let id = if index % 2 == 0 { index as i32 / 2 } else { -1 };

        Self::new(id, len)
    }

    pub fn new(id: i32, len: u32) -> Self {
        Self { id, len }
    }

    fn is_free(&self) -> bool {
        self.id == -1
    }

    fn checksum(&self, base: u32) -> u64 {
        (0..self.len)
            .map(|idx| (base + idx) as u64 * self.id as u64)
            .sum()
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

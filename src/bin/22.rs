use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<u64>().unwrap())
            .map(|i| secret_numbers(i, 2000))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut all_sequences = HashMap::new();
    for initial in input.lines().map(|l| l.parse::<u64>().unwrap()) {
        sequences(initial, 2000, &mut all_sequences);
    }

    Some(all_sequences.values().max().copied().unwrap())
}

fn secret_numbers(initial: u64, round: usize) -> u64 {
    let mut cur = initial;
    for _ in 0..round {
        cur = secret_number(cur);
    }

    cur
}

fn sequences(initial: u64, round: usize, all_sequences: &mut HashMap<[i8; 4], u64>) {
    let mut cur = initial;
    let mut sequence = VecDeque::with_capacity(4);

    let mut sequences = HashMap::<[i8; 4], u64>::new();

    let mut digit: i8 = (cur % 10) as i8;
    let mut previous_digit = digit;
    for _ in 0..round {
        cur = secret_number(cur);
        digit = (cur % 10) as i8;

        if sequence.len() == 4 {
            sequence.pop_front();
        }
        sequence.push_back(digit - previous_digit);

        if sequence.len() == 4 {
            if let Ok(arr) = sequence.make_contiguous().try_into() {
                sequences.entry(arr).or_insert(digit as u64);
            }
        }

        previous_digit = digit;
    }

    for (k, v) in sequences {
        *all_sequences.entry(k).or_default() += v;
    }
}

fn secret_number(n: u64) -> u64 {
    let mut secret = n;
    secret = mix_and_prune(secret, secret * 64);
    secret = mix_and_prune(secret, secret / 32);
    secret = mix_and_prune(secret, secret * 2048);

    secret
}

fn mix_and_prune(secret: u64, n: u64) -> u64 {
    (n ^ secret) % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_secret_numbers_of_123() {
        // GIVEN
        let mut secret = 123;

        // WHEN
        let mut numbers = Vec::new();
        for _ in 0..10 {
            secret = secret_number(secret);
            numbers.push(secret);
        }

        // THEN
        assert_eq!(
            numbers,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        )
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(
            r#"1
2
3
2024
"#,
        );
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2121)); // too low and 2140 is too high
    }
}

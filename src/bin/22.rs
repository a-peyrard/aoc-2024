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

fn secret_numbers(initial: u64, round: usize) -> u64 {
    let mut cur = initial;
    for _ in 0..round {
        cur = secret_number(cur);
    }

    cur
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

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

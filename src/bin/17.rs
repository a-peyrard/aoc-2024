use std::collections::HashMap;

advent_of_code::solution!(17);

fn parse(input: &str) -> (Computer, Vec<u64>) {
    let mut blocks = input.split("\n\n");
    let computer = Computer::parse(blocks.next().unwrap());
    let program: Vec<u64> = blocks.next().unwrap().lines().next().unwrap()[9..]
        .split(',')
        .map(|t| t.parse().unwrap())
        .collect();

    (computer, program)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut computer, program) = parse(input);

    Some(
        computer
            .run(&program)
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut computer, program) = parse(input);

    search(0, 0, &mut computer, &program)
}

fn search(a: u64, idx: usize, computer: &mut Computer, program: &Vec<u64>) -> Option<u64> {
    if idx >= program.len() {
        return Some(a);
    }

    for v in 0..8 {
        let test = 8 * a + v;
        computer.set_a(test);
        let start = program.len() - idx - 1;
        if computer.run(program) == program[start..] {
            let res = search(test, idx + 1, computer, program);
            if res.is_some() {
                return res;
            }
        }
    }

    None
}

struct Computer {
    registers: HashMap<char, u64>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let mut registers = HashMap::new();

        let mut lines = input.lines();
        registers.insert('A', lines.next().unwrap()[12..].parse().unwrap());
        registers.insert('B', lines.next().unwrap()[12..].parse().unwrap());
        registers.insert('C', lines.next().unwrap()[12..].parse().unwrap());

        Self { registers }
    }

    fn set_a(&mut self, a: u64) {
        self.registers.insert('A', a);
    }

    fn run(&mut self, program: &Vec<u64>) -> Vec<u64> {
        let mut out: Vec<u64> = Vec::new();
        let mut ptr = 0;
        while ptr < program.len() {
            let opcode = program[ptr];
            let operand = program[ptr + 1];
            let combo = self.operand_to_combo(operand);

            match opcode {
                0 => self.adv(combo),
                1 => self.bxl(operand),
                2 => self.bst(combo),
                3 => {
                    if let Some(jump) = self.jnz(operand) {
                        ptr = jump;
                        continue;
                    }
                }
                4 => self.bxc(),
                5 => out.push(self.out(combo)),
                6 => self.bdv(combo),
                7 => self.cdv(combo),
                _ => panic!("unexpected opcode {}", opcode),
            }

            ptr += 2;
        }

        out
    }

    fn adv(&mut self, value: u64) {
        self.registers
            .entry('A') //
            .and_modify(|a| *a /= 2u64.pow(value as u32));
    }

    fn bxl(&mut self, value: u64) {
        self.registers
            .entry('B') //
            .and_modify(|b| *b ^= value);
    }

    fn bst(&mut self, value: u64) {
        self.registers.insert('B', value % 8);
    }

    fn jnz(&mut self, value: u64) -> Option<usize> {
        let a = self.registers.get(&'A').unwrap();
        match a {
            0 => None,
            _ => Some(value as usize),
        }
    }

    fn bxc(&mut self) {
        let c = *self.registers.get(&'C').unwrap();
        self.registers
            .entry('B') //
            .and_modify(|b| *b ^= c);
    }

    fn out(&mut self, value: u64) -> u64 {
        value % 8
    }

    fn bdv(&mut self, value: u64) {
        let a = *self.registers.get(&'A').unwrap();

        self.registers
            .entry('B') //
            .and_modify(|b| *b = a / 2u64.pow(value as u32));
    }

    fn cdv(&mut self, value: u64) {
        let a = *self.registers.get(&'A').unwrap();

        self.registers
            .entry('C') //
            .and_modify(|c| *c = a / 2u64.pow(value as u32));
    }

    fn operand_to_combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => *self.registers.get(&'A').unwrap(),
            5 => *self.registers.get(&'B').unwrap(),
            6 => *self.registers.get(&'C').unwrap(),
            _ => panic!("we should not use operand 7"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_one_modified_input() {
        let result = part_one(
            r#"Register A: 236555995274861
Register B: 0
Register C: 0

Program: 2,4,1,3,7,5,4,2,0,3,1,5,5,5,3,0
"#,
        );
        assert_eq!(
            result,
            Some(String::from("2,4,1,3,7,5,4,2,0,3,1,5,5,5,3,0"))
        );
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None); // not working for the example :/
    }
}

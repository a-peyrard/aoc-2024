use std::collections::HashMap;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut blocks = input.split("\n\n");
    let mut computer = Computer::parse(blocks.next()?);
    let program: Vec<u32> = blocks.next().unwrap().lines().next().unwrap()[9..]
        .split(',')
        .map(|t| t.parse().unwrap())
        .collect();

    Some(computer.run(program))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

struct Computer {
    registers: HashMap<char, u32>,
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

    fn run(&mut self, program: Vec<u32>) -> String {
        let mut out: Vec<u32> = Vec::new();
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

        out.into_iter() //
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn adv(&mut self, value: u32) {
        self.registers
            .entry('A') //
            .and_modify(|a| *a /= 2u32.pow(value));
    }

    fn bxl(&mut self, value: u32) {
        self.registers
            .entry('B') //
            .and_modify(|b| *b ^= value);
    }

    fn bst(&mut self, value: u32) {
        self.registers.insert('B', value % 8);
    }

    fn jnz(&mut self, value: u32) -> Option<usize> {
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

    fn out(&mut self, value: u32) -> u32 {
        value % 8
    }

    fn bdv(&mut self, value: u32) {
        let a = *self.registers.get(&'A').unwrap();

        self.registers
            .entry('B') //
            .and_modify(|b| *b = a / 2u32.pow(value));
    }

    fn cdv(&mut self, value: u32) {
        let a = *self.registers.get(&'A').unwrap();

        self.registers
            .entry('C') //
            .and_modify(|c| *c = a / 2u32.pow(value));
    }

    fn operand_to_combo(&self, operand: u32) -> u32 {
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

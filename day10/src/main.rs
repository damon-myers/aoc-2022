use std::{collections::LinkedList, thread::current};

pub enum Operation {
    Noop,
    Add { value: i32 },
}

pub struct Instruction {
    pub operation: Operation,
    pub num_cycles: usize,
}

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

impl Instruction {
    pub fn from(line: &str) -> Self {
        if line.starts_with("addx") {
            let line_split: Vec<&str> = line.split(" ").collect();

            let parsed_number = if let [_, number] = &line_split[..] {
                number.parse().unwrap()
            } else {
                panic!("invalid number of operands for addx");
            };

            Instruction {
                operation: Operation::Add {
                    value: parsed_number,
                },
                num_cycles: 2,
            }
        } else {
            Instruction {
                operation: Operation::Noop,
                num_cycles: 1,
            }
        }
    }
}

fn input_to_instructions(input: &String) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Instruction::from)
        .collect()
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut register: i32 = 1;

    let mut current_cycle: usize = 0;

    let mut cycles_to_check: LinkedList<usize> = LinkedList::from([20, 60, 100, 140, 180, 220]);
    let mut next_cycle_check = cycles_to_check.pop_front().unwrap();

    let mut signal_strengths = Vec::new();

    for instruction in instructions {
        if current_cycle + instruction.num_cycles >= next_cycle_check {
            signal_strengths.push(register * next_cycle_check as i32);

            next_cycle_check = match cycles_to_check.pop_front() {
                Some(cycle) => cycle,
                None => break,
            }
        }

        current_cycle += instruction.num_cycles;

        match instruction.operation {
            Operation::Add { value } => register += value,
            _ => {}
        }
    }

    signal_strengths.iter().sum()
}

fn part2(instructions: &Vec<Instruction>) {
    let mut sprite_midpoint: i32 = 1;

    let mut current_cycle: usize = 0;

    let mut screen: Vec<char> = Vec::new();

    for instruction in instructions {
        for i in 1..=instruction.num_cycles {
            let pixel_being_rendered = current_cycle % (SCREEN_WIDTH);

            let sprite_pixels = (sprite_midpoint - 1)..=(sprite_midpoint + 1);

            if sprite_pixels.contains(&(pixel_being_rendered as i32)) {
                screen.push('#');
            } else {
                screen.push('.');
            }

            current_cycle += 1;
            match instruction.operation {
                Operation::Add { value } if i == instruction.num_cycles => {
                    sprite_midpoint += value;
                }
                _ => {}
            }

            println!("sprite_midpoint: {}", sprite_midpoint);

            print_screen(&screen);
        }
    }
}

fn print_screen(screen: &Vec<char>) {
    println!();
    for row in screen.chunks(SCREEN_WIDTH) {
        let joined_row: String = row.iter().collect();
        println!("{}", joined_row);
    }
    println!();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let instructions = input_to_instructions(&input);

    let result = part1(&instructions);

    println!("sum of signal strengths: {}", result);

    part2(&instructions);
}

#[cfg(test)]
mod tests {
    use crate::{input_to_instructions, part1};

    static TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        let instructions = input_to_instructions(&String::from(TEST_INPUT));

        let result = part1(&instructions);

        assert_eq!(result, 13140);
    }

    #[test]
    fn test_part2() {
        let instructions = input_to_instructions(&String::from(TEST_INPUT));

        crate::part2(&instructions);
    }
}

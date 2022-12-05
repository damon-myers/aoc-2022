use core::num;
use std::collections::LinkedList;

use regex::Regex;

pub struct Command {
    num_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

impl Command {
    // Given a line of the form:
    // "move n from x to y", parse a Command where
    // num_to_move = n, from_stack = x, to_stack = y
    // Take pattern as a parameter to avoid
    pub fn from_input_lines(lines: &Vec<&str>) -> Vec<Command> {
        let pattern =
            Regex::new(r"move (?P<num_to_move>\d+) from (?P<from_stack>\d+) to (?P<to_stack>\d+)")
                .unwrap();

        lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let matches = pattern.captures(line).unwrap();

                let num_to_move: usize = matches
                    .name("num_to_move")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();

                let from_stack: usize = matches
                    .name("from_stack")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();

                let to_stack: usize = matches.name("to_stack").unwrap().as_str().parse().unwrap();

                Command {
                    num_to_move,
                    from_stack,
                    to_stack,
                }
            })
            .collect()
    }
}

type Stacks = Vec<LinkedList<char>>;
type Commands = Vec<Command>;

fn parse_input(lines: &String) -> (Stacks, Commands) {
    let lines = lines.lines();

    let mut parsing_initial_state = true;
    let mut initial_state_lines = Vec::new();
    let mut command_lines = Vec::new();

    for line in lines {
        if line.is_empty() {
            parsing_initial_state = false;
        } else if parsing_initial_state {
            initial_state_lines.push(line);
        } else {
            command_lines.push(line);
        }
    }

    let initial_state: Stacks = build_stacks(&initial_state_lines);

    let commands: Commands = Command::from_input_lines(&command_lines);

    (initial_state, commands)
}

fn execute_commands_pt1(stacks: &mut Stacks, commands: &Commands) {
    for command in commands {
        for _ in 0..command.num_to_move {
            let element = stacks[command.from_stack - 1].pop_back().unwrap();

            stacks[command.to_stack - 1].push_back(element);
        }
    }
}

fn execute_commands_pt2(stacks: &mut Stacks, commands: &Commands) {
    for command in commands {
        let mut temp_stack = LinkedList::new();
        for _ in 0..command.num_to_move {
            let from_stack = &mut stacks[command.from_stack - 1];
            let element = from_stack.pop_back().unwrap();

            temp_stack.push_back(element);
        }

        for item in temp_stack.iter().rev() {
            let to_stack = &mut stacks[command.to_stack - 1];
            to_stack.push_back(*item);
        }
    }
}

fn get_top_of_each_stack(stacks: &Stacks) -> String {
    stacks.iter().map(|stack| stack.back().unwrap()).fold(
        String::new(),
        |mut acc: String, current_char| {
            acc.push(current_char.clone());

            acc
        },
    )
}

// The number of characters that define a column in the input
const COLUMN_SIZE: usize = 4;
fn build_stacks(initial_state_lines: &Vec<&str>) -> Stacks {
    let total_stacks = (initial_state_lines.last().unwrap().len() + 1) / 4;

    let mut stacks: Stacks = (0..total_stacks)
        .map(|_| {
            let empty_stack: LinkedList<char> = LinkedList::<char>::new();
            empty_stack
        })
        .collect();

    for stack_index in 0..total_stacks {
        let char_index = stack_index * COLUMN_SIZE + 1;

        let stack = &mut stacks[stack_index];

        // work back to front when pushing onto stacks
        for line in initial_state_lines.iter().rev() {
            // indicates a stack index line, do nothing
            if !line.contains("[") {
                continue;
            }

            let line_chars: Vec<char> = line.chars().collect();

            let char_to_push = line_chars[char_index];

            if !char_to_push.is_whitespace() {
                stack.push_back(char_to_push);
            }
        }
    }

    stacks
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (mut stacks, commands) = parse_input(&input);

    execute_commands_pt1(&mut stacks, &commands);

    println!(
        "After part1 rearranging, the top items are: {}",
        get_top_of_each_stack(&stacks)
    );

    let input = std::fs::read_to_string("input.txt").unwrap();
    let (mut stacks, commands) = parse_input(&input);

    execute_commands_pt2(&mut stacks, &commands);

    println!(
        "After part2 rearranging, the top items are: {}",
        get_top_of_each_stack(&stacks)
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1() {
        let (mut stacks, commands) = parse_input(&String::from(TEST_INPUT));

        execute_commands_pt1(&mut stacks, &commands);

        assert_eq!(get_top_of_each_stack(&stacks), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (mut stacks, commands) = parse_input(&String::from(TEST_INPUT));

        execute_commands_pt2(&mut stacks, &commands);

        assert_eq!(get_top_of_each_stack(&stacks), "MCD");
    }
}

use std::{borrow::BorrowMut, collections::HashSet};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn is_adjacent(&self, other: &Position) -> bool {
        let x_diff = other.x.abs_diff(self.x);
        let y_diff = other.y.abs_diff(self.y);

        x_diff <= 1 && y_diff <= 1
    }

    fn step_towards(&mut self, target: &Position) {
        let x_diff = target.x - self.x;
        let y_diff = target.y - self.y;

        self.x += x_diff.signum();
        self.y += y_diff.signum();
    }
}

pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub struct MovementCommand {
    pub direction: Direction,
    pub num_steps: usize,
}

fn input_to_commands(input: &String) -> Vec<MovementCommand> {
    input
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split(" ").collect();

            if let [direction, num_steps] = &split_line[..] {
                let direction = match *direction {
                    "L" => Direction::LEFT,
                    "R" => Direction::RIGHT,
                    "U" => Direction::UP,
                    "D" => Direction::DOWN,
                    _ => panic!("invalid direction"),
                };

                let num_steps = num_steps.parse().unwrap();

                MovementCommand {
                    direction,
                    num_steps,
                }
            } else {
                panic!("invalid number of chars in string");
            }
        })
        .collect()
}

// returns number of unique locations the tail visits
fn part1(commands: &Vec<MovementCommand>) -> usize {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    let mut tail_visited = HashSet::new();
    tail_visited.insert((tail.x, tail.y));

    for command in commands {
        for _ in 0..command.num_steps {
            match command.direction {
                Direction::LEFT => head.x -= 1,
                Direction::RIGHT => head.x += 1,
                Direction::UP => head.y -= 1,
                Direction::DOWN => head.y += 1,
            }

            if !tail.is_adjacent(&head) {
                tail.step_towards(&head);

                tail_visited.insert((tail.x, tail.y));
            }
        }
    }

    tail_visited.len()
}

// returns number of unique locations the tail visits
fn part2(commands: &Vec<MovementCommand>) -> usize {
    let mut nodes = vec![Position { x: 0, y: 0 }; 10];

    let mut tail_visited = HashSet::new();

    tail_visited.insert((0, 0));

    for command in commands {
        for _ in 0..command.num_steps {
            match command.direction {
                Direction::LEFT => nodes[0].x -= 1,
                Direction::RIGHT => nodes[0].x += 1,
                Direction::UP => nodes[0].y -= 1,
                Direction::DOWN => nodes[0].y += 1,
            }

            let mut new_positions = nodes.clone();
            for (index, node) in nodes.iter().enumerate().skip(1) {
                let previous_node = new_positions[index - 1];

                if !node.is_adjacent(&previous_node) {
                    let mut node_clone = node.clone();
                    node_clone.step_towards(&previous_node);

                    new_positions[index] = node_clone;
                }
            }

            nodes = new_positions;

            render(&nodes);

            tail_visited.insert((nodes[9].x, nodes[9].y));
        }
    }

    tail_visited.len()
}

fn render(rope_stuff: &Vec<Position>) {
    let min_y = rope_stuff.iter().map(|pos| pos.y).min().unwrap();
    let max_y = rope_stuff.iter().map(|pos| pos.y).max().unwrap();
    let min_x = rope_stuff.iter().map(|pos| pos.x).min().unwrap();
    let max_x = rope_stuff.iter().map(|pos| pos.x).max().unwrap();

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let mut output = vec![vec![".".to_string(); width]; height];

    for (index, position) in rope_stuff.iter().enumerate() {
        let display = match index {
            0 => "H".to_string(),
            9 => "T".to_string(),
            _ => index.to_string(),
        };

        let output_row = (position.y - min_y) as usize;
        let output_col = (position.x - min_x) as usize;

        output[output_row][output_col] = display;
    }

    println!();

    for line in output {
        println!("{}", line.join(""));
    }

    println!();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let commands = input_to_commands(&input);

    let num_visited = part1(&commands);

    println!("visited pt1: {}", num_visited);

    let num_visited = part2(&commands);

    println!("visited pt2: {}", num_visited);
}

#[cfg(test)]
mod tests {
    use core::num;

    use crate::input_to_commands;

    static TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static TEST2_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1() {
        let commands = input_to_commands(&String::from(TEST_INPUT));

        let num_visited = crate::part1(&commands);

        assert_eq!(num_visited, 13);
    }

    #[test]
    fn part2() {
        let commands = input_to_commands(&String::from(TEST2_INPUT));

        let num_visited = crate::part2(&commands);

        assert_eq!(num_visited, 36);
    }
}

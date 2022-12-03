#[derive(Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    pub fn encrypted_to_tuple(col1: &str, col2: &str) -> Option<(Self, Self)> {
        let choice1 = match col1 {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => return None,
        };

        let choice2 = match col2 {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => return None,
        };

        Some((choice1, choice2))
    }

    pub fn encrypted_to_tuple_pt2(col1: &str, col2: &str) -> Option<(Self, Self)> {
        let their_choice = match col1 {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => return None,
        };

        let result = match col2 {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => return None,
        };

        let my_choice = match (result, their_choice) {
            (Result::Win, Choice::Rock) => Choice::Paper,
            (Result::Win, Choice::Paper) => Choice::Scissors,
            (Result::Win, Choice::Scissors) => Choice::Rock,
            (Result::Lose, Choice::Rock) => Choice::Scissors,
            (Result::Lose, Choice::Paper) => Choice::Rock,
            (Result::Lose, Choice::Scissors) => Choice::Paper,
            (Result::Draw, their_choice) => their_choice,
        };

        Some((their_choice, my_choice))
    }
}

pub enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    pub fn score(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}

pub struct Round {
    their_choice: Choice,
    my_choice: Choice,
}

impl Round {
    pub fn get_result(&self) -> Result {
        use crate::Choice::{Paper, Rock, Scissors};

        match (&self.their_choice, &self.my_choice) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Result::Win,
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Result::Lose,
            _ => Result::Draw,
        }
    }

    pub fn score(&self) -> u32 {
        &self.my_choice.score() + &self.get_result().score()
    }
}

fn input_to_rounds(input: &String) -> Vec<Round> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let columns: Vec<&str> = line.split(' ').collect();

            let (col1, col2) = match &columns[..] {
                &[col1, col2, ..] => (col1, col2),
                _ => panic!("too few columns in input line: {}", &line),
            };

            Choice::encrypted_to_tuple(col1, col2).unwrap()
        })
        .map(|(their_pick, my_pick)| Round {
            their_choice: their_pick,
            my_choice: my_pick,
        })
        .collect()
}

fn input_to_rounds_pt2(input: &String) -> Vec<Round> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let columns: Vec<&str> = line.split(' ').collect();

            let (col1, col2) = match &columns[..] {
                &[col1, col2, ..] => (col1, col2),
                _ => panic!("too few columns in input line: {}", &line),
            };

            Choice::encrypted_to_tuple_pt2(col1, col2).unwrap()
        })
        .map(|(their_pick, my_pick)| Round {
            their_choice: their_pick,
            my_choice: my_pick,
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt");

    let rounds = input_to_rounds(&input.unwrap());

    let total_score: u32 = rounds.iter().map(|round| round.score()).sum();

    println!("Total score from pt1: {}", total_score);

    let input = std::fs::read_to_string("input.txt");

    let rounds = input_to_rounds_pt2(&input.unwrap());

    let total_score: u32 = rounds.iter().map(|round| round.score()).sum();

    println!("Total score from pt2: {}", total_score);
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TEST_INPUT: &str = "
A Y
B X
C Z";

    #[test]
    fn part1() {
        let rounds = input_to_rounds(&String::from(TEST_INPUT));

        let total_score: u32 = rounds.iter().map(|round| round.score()).sum();

        assert_eq!(total_score, 15);
    }

    #[test]
    fn part2() {
        let rounds = input_to_rounds_pt2(&String::from(TEST_INPUT));

        let total_score: u32 = rounds.iter().map(|round| round.score()).sum();

        assert_eq!(total_score, 12);
    }
}

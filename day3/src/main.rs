use std::collections::HashSet;

pub struct Rucksack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
}

impl Rucksack {
    pub fn from(line: &str) -> Self {
        let chars: Vec<char> = line.chars().collect();

        let total_items = chars.len();

        let mut first_compartment = HashSet::new();
        let mut second_compartment = HashSet::new();

        for (index, item) in chars.iter().enumerate() {
            if index < (total_items / 2) {
                first_compartment.insert(item.clone());
            } else {
                second_compartment.insert(item.clone());
            }
        }

        Rucksack {
            first_compartment,
            second_compartment,
        }
    }

    pub fn get_all_items(&self) -> HashSet<char> {
        HashSet::from_iter(
            self.first_compartment
                .union(&self.second_compartment)
                .cloned(),
        )
    }

    pub fn get_common_item(&self) -> Option<&char> {
        self.first_compartment
            .intersection(&self.second_compartment)
            .next()
    }
}

fn calculate_priority(item: &char) -> u16 {
    match item {
        'a' => 1,
        'a'..='z' => (*item as u16) - ('a' as u16) + 1,
        'A' => 27,
        'A'..='Z' => (*item as u16) - ('A' as u16) + 27,
        _ => panic!("unsupported character"),
    }
}

fn part2(rucksacks: &Vec<Rucksack>) -> u16 {
    let mut group_start_index = 0;
    let mut group_end_index = 2;

    let mut group_priorities = Vec::new();

    while group_end_index < rucksacks.len() {
        let group = &rucksacks[group_start_index..=group_end_index];

        let group_items: Vec<HashSet<char>> = group
            .iter()
            .map(|rucksack| rucksack.get_all_items())
            .collect();

        println!("group: {:?}", group_items);

        let common_set = group
            .iter()
            .map(|rucksack| rucksack.get_all_items())
            .reduce(|acc, other| HashSet::from_iter(acc.intersection(&other).cloned()))
            .unwrap();

        println!("common_set: {:?}", common_set);

        let common_item = common_set.iter().next().unwrap(); // should only be one element

        group_priorities.push(calculate_priority(common_item));

        group_start_index += 3;
        group_end_index += 3;
    }

    let priority_sum: u16 = group_priorities.iter().sum();

    println!("Part 2 sum: {}", priority_sum);

    priority_sum
}

fn main() {
    let rucksacks: Vec<Rucksack> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(Rucksack::from)
        .collect();

    let score: u16 = rucksacks
        .iter()
        .map(|rucksack| rucksack.get_common_item().unwrap())
        .map(calculate_priority)
        .sum();

    println!("Score: {}", score);

    part2(&rucksacks);
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TEST_INPUT: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part1() {
        let rucksacks: Vec<Rucksack> = TEST_INPUT
            .lines()
            .filter(|line| !line.is_empty())
            .map(Rucksack::from)
            .collect();

        for rucksack in &rucksacks {
            println!("First compartment: {:?}", rucksack.first_compartment);
            println!("Second compartment: {:?}", rucksack.second_compartment);
            println!("Common item: {:?}", rucksack.get_common_item());
            println!();
        }

        let score: u16 = rucksacks
            .iter()
            .map(|rucksack| rucksack.get_common_item().unwrap())
            .map(calculate_priority)
            .sum();

        assert_eq!(score, 157)
    }

    #[test]
    fn test_part2() {
        let rucksacks: Vec<Rucksack> = TEST_INPUT
            .lines()
            .filter(|line| !line.is_empty())
            .map(Rucksack::from)
            .collect();

        let sum = part2(&rucksacks);

        assert_eq!(sum, 70);
    }
}

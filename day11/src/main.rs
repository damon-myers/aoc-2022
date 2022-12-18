use std::collections::LinkedList;

pub struct Monkey {
    pub items: LinkedList<u128>,
    pub num_inspections: u128,
    pub inspect_item: Box<dyn Fn(&u128) -> u128>,
    pub test_number: u128, // is the worry level divisible by this amount?
    pub target_monkey: Box<dyn Fn(bool) -> usize>,
    pub stress_management_number: u128, // multiply all of the divisors together, and use this to keep numbers from overflowing a 128 bit integer
}

impl Monkey {
    pub fn is_empty_handed(&self) -> bool {
        self.items.is_empty()
    }

    pub fn inspect_items_in_hand_part1(&mut self) {
        let new_worry_levels: LinkedList<u128> = self
            .items
            .iter()
            .map(|worry_level| {
                self.num_inspections += 1;
                (self.inspect_item)(worry_level) / 3
            })
            .collect();

        self.items = new_worry_levels;
    }

    pub fn inspect_items_in_hand_part2(&mut self) {
        let new_worry_levels: LinkedList<u128> = self
            .items
            .iter()
            .map(|worry_level| {
                self.num_inspections += 1;
                (self.inspect_item)(worry_level) % self.stress_management_number
            })
            .collect();

        self.items = new_worry_levels;
    }

    pub fn determine_target_monkeys(&self) -> Vec<usize> {
        self.items
            .iter()
            .map(|item_worry| self.test_item(*item_worry))
            .map(|test_result| (self.target_monkey)(test_result))
            .collect()
    }

    pub fn throw_items(&mut self) -> Vec<u128> {
        let items: Vec<u128> = self.items.iter().map(|item| item.clone()).collect();

        self.items = LinkedList::new();

        items
    }

    pub fn catch_item(&mut self, item_worry: u128) {
        self.items.push_back(item_worry);
    }

    pub fn test_item(&self, worry_level: u128) -> bool {
        worry_level % self.test_number == 0
    }
}

fn parse_starting_items(item_line: &str) -> LinkedList<u128> {
    let input_split: Vec<&str> = item_line.split(":").collect();

    input_split[1]
        .split(",")
        .map(|worry_amount| worry_amount.trim())
        .map(|worry_amount| worry_amount.parse().unwrap())
        .collect()
}

fn parse_operation(operation_line: &String) -> Box<dyn Fn(&u128) -> u128> {
    let input_split: Vec<&str> = operation_line.split("new = ").collect();

    let operation_split: Vec<&str> = input_split[1].split(" ").collect();

    let (operator, operand) = if let [_, operator, operand] = &operation_split[..] {
        (operator, operand)
    } else {
        panic!(
            "invalid number of arguments on operation_line: {}",
            operation_line
        );
    };

    match (*operator, *operand) {
        ("*", "old") => Box::new(move |old| (old * old)),
        ("+", "old") => Box::new(move |old| (old + old)),
        ("*", operand) => {
            let parsed_operand = operand.parse::<u128>().unwrap();
            Box::new(move |old| (old * parsed_operand))
        }
        ("+", operand) => {
            let parsed_operand = operand.parse::<u128>().unwrap();
            Box::new(move |old| old + parsed_operand)
        }
        (operation, operand) => {
            panic!(
                "unsupported operation! Tried to {} on {}",
                operation, operand
            );
        }
    }
}

fn parse_test_number(test_line: &String) -> u128 {
    let input_split: Vec<&str> = test_line.split("divisible by ").collect();

    let number = if let [_, number, ..] = &input_split[..] {
        number
    } else {
        panic!("couldn't find number in test_line: {}", test_line);
    };

    number.parse().unwrap()
}

fn parse_target_monkey(true_line: &String, false_line: &String) -> Box<dyn Fn(bool) -> usize> {
    let true_split: Vec<&str> = true_line.split("throw to monkey").collect();
    let false_split: Vec<&str> = false_line.split("throw to monkey").collect();

    let true_target = if let [_, number, ..] = &true_split[..] {
        number
    } else {
        panic!("couldn't find target monkey in true_line: {}", true_line);
    };

    let false_target = if let [_, number, ..] = &false_split[..] {
        number
    } else {
        panic!("couldn't find target monkey in false_line: {}", false_line);
    };

    let parsed_true_target: usize = true_target.trim().parse().unwrap();
    let parsed_false_target: usize = false_target.trim().parse().unwrap();

    Box::new(move |test_result| {
        if test_result {
            parsed_true_target
        } else {
            parsed_false_target
        }
    })
}

fn initialize_monkeys(input: &String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let input_lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    for chunk in input_lines[..].chunks(6) {
        let monkey_description: Vec<String> = chunk.iter().map(|line| line.to_string()).collect();

        monkeys.push(Monkey {
            items: parse_starting_items(&monkey_description[1]),
            num_inspections: 0,
            inspect_item: parse_operation(&monkey_description[2]),
            test_number: parse_test_number(&monkey_description[3]),
            target_monkey: parse_target_monkey(&monkey_description[4], &monkey_description[5]),
            stress_management_number: 0, // will set after all have been parsed
        });
    }

    let stress_management_number: u128 = monkeys.iter().map(|monkey| monkey.test_number).product();

    for monkey in monkeys.iter_mut() {
        monkey.stress_management_number = stress_management_number;
    }

    monkeys
}

fn part1(input: &String) -> u128 {
    let mut monkeys = initialize_monkeys(input);

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let current_monkey = &mut monkeys[monkey_index];

            if current_monkey.is_empty_handed() {
                continue;
            }

            current_monkey.inspect_items_in_hand_part1();

            let target_monkeys = current_monkey.determine_target_monkeys();
            let items = current_monkey.throw_items();

            for (target, item) in target_monkeys.iter().zip(items.iter()) {
                monkeys[*target].catch_item(item.clone());
            }
        }
    }

    monkeys.sort_by(|monkey1, monkey2| monkey2.num_inspections.cmp(&monkey1.num_inspections));

    monkeys
        .iter()
        .take(2)
        .map(|monkey| {
            println!("monkey did {} inspections", monkey.num_inspections);
            monkey.num_inspections
        })
        .product()
}

fn part2(input: &String) -> u128 {
    let mut monkeys = initialize_monkeys(input);

    for iteration in 0..10_000 {
        if iteration % 1_000 == 0 {
            println!("reached iteration: {}", iteration)
        }

        for monkey_index in 0..monkeys.len() {
            let current_monkey = &mut monkeys[monkey_index];

            if current_monkey.is_empty_handed() {
                continue;
            }

            current_monkey.inspect_items_in_hand_part2();

            let target_monkeys = current_monkey.determine_target_monkeys();
            let items = current_monkey.throw_items();

            for (target, item) in target_monkeys.iter().zip(items.iter()) {
                monkeys[*target].catch_item(item.clone());
            }
        }
    }

    monkeys.sort_by(|monkey1, monkey2| monkey2.num_inspections.cmp(&monkey1.num_inspections));

    monkeys
        .iter()
        .take(2)
        .map(|monkey| {
            println!("monkey did {} inspections", monkey.num_inspections);
            monkey.num_inspections
        })
        .product()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let monkey_business = crate::part1(&input);

    println!("total monkey business: {}", monkey_business);

    let monkey_business = crate::part2(&input);

    println!("total monkey business: {}", monkey_business);
}

#[cfg(test)]
mod tests {
    static TEST_STRING: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        let input = String::from(TEST_STRING);

        let monkey_business = crate::part1(&input);

        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn test_part2() {
        let input = String::from(TEST_STRING);

        let monkey_business = crate::part2(&input);

        assert_eq!(monkey_business, 2713310158u128);
    }
}

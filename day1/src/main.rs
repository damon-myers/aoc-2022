#[derive(Clone)]
pub struct FoodItem {
    calories: u32,
}

#[derive(Clone)]
pub struct Elf {
    pub items: Vec<FoodItem>,
}

impl Elf {
    fn new() -> Self {
        Elf { items: Vec::new() }
    }

    pub fn get_total_calories(&self) -> u32 {
        self.items.iter().map(|food| food.calories).sum()
    }
}

pub fn input_to_elves(input: &str) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();

    let mut current_elf_index: usize = 0;
    input.lines().for_each(|line| {
        if elves.is_empty() {
            elves.push(Elf::new());
        }

        let current_elf = &mut elves[current_elf_index];

        if line.is_empty() {
            (&mut elves).push(Elf::new());
            current_elf_index = current_elf_index + 1;
        } else {
            let current_item_calories = line.parse::<u32>().unwrap();

            current_elf.items.push(FoodItem {
                calories: current_item_calories,
            });
        }
    });

    elves
}

pub fn elf_with_most(elves: &Vec<Elf>) -> &Elf {
    elves
        .iter()
        .max_by(|elf1, elf2| elf1.get_total_calories().cmp(&elf2.get_total_calories()))
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt");
    let mut elves = input_to_elves(&input.unwrap());

    let elf_with_most = elf_with_most(&elves);

    println!(
        "Elf with most had {} calories",
        elf_with_most.get_total_calories()
    );

    // part two
    // sort the elves by calories
    elves.sort_by(|elf1, elf2| elf1.get_total_calories().cmp(&elf2.get_total_calories()));
    elves.reverse();

    let top_3_calories: Vec<u32> = elves[0..3]
        .iter()
        .map(|elf| elf.get_total_calories())
        .collect();

    let sum_of_top_3: u32 = top_3_calories.iter().sum();

    println!("Calories of the top 3 elves are: {:?}", top_3_calories);
    println!("Summed: {}", sum_of_top_3);
}

#[cfg(test)]
mod tests {
    use crate::{elf_with_most, input_to_elves};

    static TEST_INPUT: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn it_works() {
        let mut elves = input_to_elves(TEST_INPUT);

        let elf_with_most = elf_with_most(&elves);

        assert_eq!(elf_with_most.get_total_calories(), 24000);

        // sort the elves by calories
        elves.sort_by(|elf1, elf2| elf1.get_total_calories().cmp(&elf2.get_total_calories()));
        elves.reverse();

        let top_3_calories: Vec<u32> = elves[0..3]
            .iter()
            .map(|elf| elf.get_total_calories())
            .collect();

        let sum_of_top_3: u32 = top_3_calories.iter().sum();

        assert_eq!(sum_of_top_3, 45000);
    }
}

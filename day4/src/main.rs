pub struct SectionAssignment {
    begin_section: u32,
    end_section: u32,
}

impl SectionAssignment {
    // Given input string of "1-2", return a SectionAssignment with begin: 1, end: 2
    pub fn from_input_string(input: &str) -> Self {
        let input_chars: Vec<&str> = input.split("-").collect();

        println!("{:?}", input_chars);

        if let [begin_section, end_section] = &input_chars[..] {
            SectionAssignment {
                begin_section: begin_section.parse().unwrap(),
                end_section: end_section.parse().unwrap(),
            }
        } else {
            panic!("Too many hyphens in section assignment");
        }
    }

    pub fn fully_contains(&self, other_assignment: &SectionAssignment) -> bool {
        self.begin_section <= other_assignment.begin_section
            && self.end_section >= other_assignment.end_section
    }

    pub fn overlaps(&self, other_assignment: &SectionAssignment) -> bool {
        let other_begin_in_range = other_assignment.begin_section >= self.begin_section
            && other_assignment.begin_section <= self.end_section;
        let other_end_in_range = other_assignment.end_section >= self.begin_section
            && other_assignment.end_section <= self.end_section;
        let other_fully_contains = other_assignment.fully_contains(self);

        other_begin_in_range || other_end_in_range || other_fully_contains
    }
}

fn input_to_pairs(input: &String) -> Vec<(SectionAssignment, SectionAssignment)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let assignments: Vec<&str> = line.split(",").collect();

            if let [first_assignment, second_assignment] = &assignments[..] {
                (
                    SectionAssignment::from_input_string(first_assignment),
                    SectionAssignment::from_input_string(second_assignment),
                )
            } else {
                panic!("Too many items in input line");
            }
        })
        .collect()
}

fn total_fully_overlapping(assignments: &Vec<(SectionAssignment, SectionAssignment)>) -> u32 {
    assignments
        .iter()
        .map(|(assignment1, assignment2)| {
            if (assignment1.fully_contains(&assignment2)
                || assignment2.fully_contains(&assignment1))
            {
                1
            } else {
                0
            }
        })
        .sum()
}

fn total_partially_overlapping(assignments: &Vec<(SectionAssignment, SectionAssignment)>) -> u32 {
    assignments
        .iter()
        .map(|(assignment1, assignment2)| {
            if assignment1.overlaps(&assignment2) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let assignments = input_to_pairs(&input);

    let total_overlapping = total_fully_overlapping(&assignments);

    println!("Total overlapping: {}", total_overlapping);

    let total_partial_overlapping = total_partially_overlapping(&assignments);

    println!("Total partial overlapping: {}", total_partial_overlapping);
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TEST_INPUT: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1() {
        let assignments = input_to_pairs(&String::from(TEST_INPUT));

        let total_overlapping = total_fully_overlapping(&assignments);

        println!("Total overlapping: {}", total_overlapping);

        assert_eq!(total_overlapping, 2);
    }

    #[test]
    fn test_part2() {
        let assignments = input_to_pairs(&String::from(TEST_INPUT));

        let total_overlapping = total_partially_overlapping(&assignments);

        println!("Total overlapping: {}", total_overlapping);

        assert_eq!(total_overlapping, 4);
    }
}

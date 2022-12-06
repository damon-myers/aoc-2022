use itertools::Itertools;

fn first_non_repeating_buffer(input: &String, buffer_size: usize) -> usize {
    let is_non_repeating = |buffer: &str| {
        let buffer_chars: Vec<char> = buffer.chars().unique().collect();

        buffer_chars.len() == buffer.len()
    };

    let mut start_index: usize = 0;
    let mut end_index: usize = buffer_size;

    let mut buffer = &input[start_index..end_index];
    while !is_non_repeating(buffer) {
        start_index += 1;
        end_index += 1;

        buffer = &input[start_index..end_index];
    }

    println!("start of packet: {}", buffer);
    end_index
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let num_chars = first_non_repeating_buffer(&input, 4);

    println!("Read {} chars before finding start of packet", num_chars);

    let num_chars = first_non_repeating_buffer(&input, 14);

    println!("Read {} chars before finding start of message", num_chars);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let test_inputs: Vec<(&str, usize)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, expected) in test_inputs {
            let actual = first_non_repeating_buffer(&String::from(input), 4);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_inputs: Vec<(&str, usize)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, expected) in test_inputs {
            let actual = first_non_repeating_buffer(&String::from(input), 14);

            assert_eq!(actual, expected);
        }
    }
}

use std::collections::HashMap;

use priority_queue::DoublePriorityQueue;

type Position = (usize, usize);

pub struct Map {
    heightmap: Vec<Vec<u8>>,
}

impl Map {
    pub fn shortest_path(
        &self,
        start_position: Position,
        target_position: Position,
    ) -> Option<Vec<Position>> {
        let mut search_frontier: DoublePriorityQueue<Position, u16> = DoublePriorityQueue::new();

        // map from node in path -> the node before that one in the path
        let mut previous_positions: HashMap<Position, Option<Position>> = HashMap::new();

        let mut current_cost: HashMap<Position, u16> = HashMap::new();

        search_frontier.push(start_position, 0);
        previous_positions.insert(start_position, None);
        current_cost.insert(start_position, 0);

        while !search_frontier.is_empty() {
            let (current, _) = search_frontier.pop_min().unwrap();

            if current == target_position {
                break;
            }

            let possible_neighbors = self.get_potential_neighbors(current);

            for neighbor in possible_neighbors {
                match neighbor {
                    Some(neighbor) => {
                        let cost_to_neighbor = current_cost[&current] + 1;
                        if !current_cost.contains_key(&neighbor)
                            || cost_to_neighbor < current_cost[&neighbor]
                        {
                            current_cost.insert(neighbor, cost_to_neighbor);
                            search_frontier.push(neighbor, cost_to_neighbor);
                            previous_positions.insert(neighbor, Some(current));
                        }
                    }
                    None => continue,
                }
            }
        }

        let mut path = Vec::new();

        let current = previous_positions.get(&target_position)?;

        let mut current = current.unwrap(); // only start will be None

        path.push(current);

        while current != start_position {
            current = previous_positions[&current].unwrap();
            path.push(current);
        }

        Some(path.iter().rev().map(|position| *position).collect())
    }

    //
    pub fn get_potential_neighbors(&self, position: Position) -> [Option<Position>; 4] {
        let map_width = self.heightmap.first().unwrap().len();
        let map_height = self.heightmap.len();

        let is_at_top_edge = position.1 == 0;
        let is_at_right_edge = position.0 == map_width - 1;
        let is_at_bottom_edge = position.1 == map_height - 1;
        let is_at_left_edge = position.0 == 0;

        let top_neighbor = if is_at_top_edge {
            None
        } else {
            Some((position.0, position.1 - 1))
        };
        let right_neighbor = if is_at_right_edge {
            None
        } else {
            Some((position.0 + 1, position.1))
        };
        let bottom_neighbor = if is_at_bottom_edge {
            None
        } else {
            Some((position.0, position.1 + 1))
        };
        let left_neighbor = if is_at_left_edge {
            None
        } else {
            Some((position.0 - 1, position.1))
        };

        let current_height = self.heightmap[position.1][position.0];

        let check_height = |neighbor: Position| {
            let neighbor_height = self.heightmap[neighbor.1][neighbor.0];
            if neighbor_height <= current_height + 1 {
                Some(neighbor)
            } else {
                None
            }
        };

        // counter-clockwise starting from top
        [
            top_neighbor.map_or(None, check_height),
            right_neighbor.map_or(None, check_height),
            bottom_neighbor.map_or(None, check_height),
            left_neighbor.map_or(None, check_height),
        ]
    }

    pub fn lowest_points(&self) -> Vec<Position> {
        let lowest_height = 'a' as u8;

        let mut positions = Vec::new();
        for (row_idx, row) in self.heightmap.iter().enumerate() {
            for (col_idx, height) in row.iter().enumerate() {
                if *height == lowest_height {
                    positions.push((col_idx, row_idx));
                }
            }
        }

        positions
    }
}

fn parse_map(input: &String) -> (Map, Position, Position) {
    let parse_heights = |line: &str| {
        line.chars()
            .map(|character| match character {
                'S' => 'a' as u8,
                'E' => 'z' as u8,
                val => val as u8,
            })
            .collect()
    };

    let heightmap: Vec<Vec<u8>> = input.lines().map(|line| parse_heights(line)).collect();

    let mut start_position = (0, 0);
    let mut target_position = (0, 0);
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, character) in row.chars().enumerate() {
            if character == 'S' {
                start_position = (col_idx, row_idx);
            } else if character == 'E' {
                target_position = (col_idx, row_idx);
            }
        }
    }

    (Map { heightmap }, start_position, target_position)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (map, start_pos, end_pos) = parse_map(&input);

    let shortest_path = map.shortest_path(start_pos, end_pos);

    println!(
        "Part1- Shortest path to destination: {}",
        shortest_path.unwrap().len()
    );

    let low_positions = map.lowest_points();

    let mut paths = Vec::new();
    for low_pos in low_positions {
        if let Some(path) = map.shortest_path(low_pos, end_pos) {
            paths.push(path);
        }
    }

    paths.sort_by(|path1, path2| path1.len().cmp(&path2.len()));

    println!("Part2 - Shortest of paths: {}", paths[0].len());
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TEST_STRING: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        let input = String::from(TEST_STRING);
        let (map, start_pos, end_pos) = parse_map(&input);

        let shortest_path = map.shortest_path(start_pos, end_pos);

        assert_eq!(shortest_path.unwrap().len(), 31);
    }

    #[test]
    fn test_part2() {
        let input = String::from(TEST_STRING);
        let (map, _start_pos, end_pos) = parse_map(&input);

        let low_positions = map.lowest_points();

        let mut paths = Vec::new();
        for low_pos in low_positions {
            if let Some(path) = map.shortest_path(low_pos, end_pos) {
                paths.push(path);
            }
        }

        paths.sort_by(|path1, path2| path1.len().cmp(&path2.len()));

        assert_eq!(paths[0].len(), 29)
    }
}

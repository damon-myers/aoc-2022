pub struct Forest {
    tree_heights: Vec<Vec<u16>>,
    width: usize, // number of columns
    depth: usize, // number of rows
}

impl Forest {
    pub fn from_input(input: &String) -> Self {
        let tree_heights: Vec<Vec<u16>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parsed_heights: Vec<u16> =
                    line.trim().chars().map(|digit| digit as u16).collect();

                parsed_heights
            })
            .collect();

        let width = if let Some(first_row) = tree_heights.first() {
            first_row.len()
        } else {
            0
        };

        let depth = tree_heights.len();

        Forest {
            width,
            depth,
            tree_heights,
        }
    }

    pub fn create_visibility_map(&self) -> Vec<Vec<bool>> {
        let mut visibility = vec![vec![false; self.width]; self.depth];

        for (row_index, row) in self.tree_heights.iter().enumerate() {
            for (col_index, tree_height) in row.iter().enumerate() {
                let is_left_edge = col_index == 0;
                let is_right_edge = col_index == self.width - 1;
                let is_top_edge = row_index == 0;
                let is_bottom_edge = row_index == self.depth - 1;

                if is_left_edge || is_right_edge || is_top_edge || is_bottom_edge {
                    visibility[row_index][col_index] = true;
                    continue;
                }

                let is_left_visible = row[0..col_index].iter().all(|height| height < tree_height);
                let is_right_visible = row[(col_index + 1)..self.width]
                    .iter()
                    .all(|height| height < tree_height);
                let is_top_visible = self.tree_heights[0..row_index]
                    .iter()
                    .map(|row| row[col_index])
                    .all(|height| height < *tree_height);
                let is_bottom_visible = self.tree_heights[(row_index + 1)..self.depth]
                    .iter()
                    .map(|row| row[col_index])
                    .all(|height| height < *tree_height);

                visibility[row_index][col_index] =
                    is_left_visible || is_right_visible || is_top_visible || is_bottom_visible;
            }
        }

        visibility
    }

    pub fn create_scenic_score_map(&self) -> Vec<Vec<usize>> {
        let mut scenic_scores = vec![vec![0; self.width]; self.depth];

        for (row_index, row) in self.tree_heights.iter().enumerate() {
            for (col_index, tree_height) in row.iter().enumerate() {
                let is_left_edge = col_index == 0;
                let is_right_edge = col_index == self.width - 1;
                let is_top_edge = row_index == 0;
                let is_bottom_edge = row_index == self.depth - 1;

                let left_score = if is_left_edge {
                    0
                } else {
                    let unblocked_trees = row[0..col_index]
                        .iter()
                        .rev()
                        .take_while(|height| *height < tree_height)
                        .count();

                    // can see to the edge of the clearing
                    if unblocked_trees == row[0..col_index].len() {
                        unblocked_trees
                    } else {
                        unblocked_trees + 1 // add one for the tree blocking our sight
                    }
                };

                let right_score = if is_right_edge {
                    0
                } else {
                    let unblocked_trees = row[(col_index + 1)..self.width]
                        .iter()
                        .take_while(|height| *height < tree_height)
                        .count();

                    // can see to the edge of the clearing
                    if unblocked_trees == row[(col_index + 1)..self.width].len() {
                        unblocked_trees
                    } else {
                        unblocked_trees + 1 // add one for the tree blocking our sight
                    }
                };

                let top_score = if is_top_edge {
                    0
                } else {
                    let unblocked_trees = self.tree_heights[0..row_index]
                        .iter()
                        .map(|row| row[col_index])
                        .rev()
                        .take_while(|height| height < tree_height)
                        .count();

                    // can see to the edge of the clearing
                    if unblocked_trees == self.tree_heights[0..row_index].len() {
                        unblocked_trees
                    } else {
                        unblocked_trees + 1 // add one for the tree blocking our sight
                    }
                };

                let bottom_score = if is_bottom_edge {
                    0
                } else {
                    let unblocked_trees = self.tree_heights[(row_index + 1)..self.depth]
                        .iter()
                        .map(|row| row[col_index])
                        .take_while(|height| height < tree_height)
                        .count();

                    // can see to the edge of the clearing
                    if unblocked_trees == self.tree_heights[(row_index + 1)..self.depth].len() {
                        unblocked_trees
                    } else {
                        unblocked_trees + 1 // add one for the tree blocking our sight
                    }
                };

                // println!(
                //     "row: {}, col: {}, left: {}, right: {}, top: {}, bot: {}",
                //     row_index, col_index, left_score, right_score, top_score, bottom_score
                // );

                scenic_scores[row_index][col_index] =
                    left_score * right_score * top_score * bottom_score;
            }
        }

        scenic_scores
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let forest = Forest::from_input(&input);

    let visibility = forest.create_visibility_map();

    let visible_count: usize = visibility
        .iter()
        .map(|row| row.iter().filter(|is_visible| **is_visible).count())
        .sum();

    println!("visible count: {}", visible_count);

    let scenic_scores = forest.create_scenic_score_map();

    let max_score = scenic_scores
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();

    println!("max scenic score: {}", *max_score);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    use crate::*;

    #[test]
    fn part1() {
        let forest = Forest::from_input(&String::from(TEST_INPUT));

        let visibility = forest.create_visibility_map();

        let visible_count: usize = visibility
            .iter()
            .map(|row| row.iter().filter(|is_visible| **is_visible).count())
            .sum();

        assert_eq!(visible_count, 21);
    }

    #[test]
    fn part2() {
        let forest = Forest::from_input(&String::from(TEST_INPUT));

        let scenic_scores = forest.create_scenic_score_map();

        println!("scenic_scores:\n{:#?}", scenic_scores);

        let max_score = scenic_scores
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap();

        assert_eq!(*max_score, 8);
    }
}

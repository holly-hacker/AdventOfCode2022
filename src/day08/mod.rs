use super::*;

pub struct Day;

impl AocDay<usize> for Day {
    const DAY: u32 = 8;

    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let (trees, width, height) = parse_input(input);

        let outer_ring_size = width * 2 + height * 2 - 4;

        // iterate over inner trees
        let mut visible_trees = 0;
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let current_tree = trees[y][x];
                // TODO: can cache biggest tree to each side

                // check LOS left
                let mut tree_covered = false;
                for x_check in (0..x).rev() {
                    if current_tree <= trees[y][x_check] {
                        // tree is covered
                        tree_covered = true;
                        continue;
                    }
                }
                if !tree_covered {
                    visible_trees += 1;
                    continue;
                }

                // check LOS right
                let mut tree_covered = false;
                for x_check in (x + 1)..width {
                    if current_tree <= trees[y][x_check] {
                        // tree is covered
                        tree_covered = true;
                        continue;
                    }
                }
                if !tree_covered {
                    visible_trees += 1;
                    continue;
                }

                // check LOS up
                let mut tree_covered = false;
                for y_check in (0..y).rev() {
                    if current_tree <= trees[y_check][x] {
                        // tree is covered
                        tree_covered = true;
                        continue;
                    }
                }
                if !tree_covered {
                    visible_trees += 1;
                    continue;
                }

                // check LOS down
                let mut tree_covered = false;
                for y_check in (y + 1)..height {
                    if current_tree <= trees[y_check][x] {
                        // tree is covered
                        tree_covered = true;
                        continue;
                    }
                }
                if !tree_covered {
                    visible_trees += 1;
                    continue;
                }
            }
        }

        visible_trees + outer_ring_size
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let (trees, width, height) = parse_input(input);

        let mut highest_score = 0;

        // outside trees will always be 0 because of the multiplication
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let start_tree = trees[y][x];
                // println!("Current tree: {start_tree}");
                let mut score = 1;

                // left
                let mut score_calc = 0;
                for x_check in (0..x).rev() {
                    let tree = trees[y][x_check];

                    score_calc += 1;

                    if tree >= start_tree {
                        break;
                    }
                }
                // println!("{x},{y} score left: {score_calc}");
                score *= score_calc;

                // right
                let mut score_calc = 0;
                for x_check in (x + 1)..width {
                    let tree = trees[y][x_check];

                    score_calc += 1;

                    if tree >= start_tree {
                        break;
                    }
                }
                // println!("{x},{y} score right:{score_calc}");
                score *= score_calc;

                // up
                let mut score_calc = 0;
                for y_check in (0..y).rev() {
                    let tree = trees[y_check][x];

                    score_calc += 1;

                    if tree >= start_tree {
                        break;
                    }
                }
                // println!("{x},{y} score up:   {score_calc}");
                score *= score_calc;

                // down
                let mut score_calc = 0;
                for y_check in (y + 1)..height {
                    let tree = trees[y_check][x];
                    score_calc += 1;
                    if tree >= start_tree {
                        break;
                    }
                }
                // println!("{x},{y} score down: {score_calc}");
                score *= score_calc;

                highest_score = highest_score.max(score);
            }
        }

        highest_score
    }
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, usize, usize) {
    let bytes = input.as_bytes();
    let width = bytes.iter().position(|&b| b == b'\n').unwrap();
    let mut trees = vec![];
    let mut bytes_index = 0;
    while bytes_index < bytes.len() {
        let mut vec = vec![0; width];

        for vec_index in 0..width {
            vec[vec_index] = bytes[bytes_index] - b'0';
            bytes_index += 1; // TODO: maybe you can save a few instructions by not incrementing and adding to bytes_index
        }

        bytes_index += 1;

        trees.push(vec);
    }

    let height = trees.len();
    debug_assert_eq!(height * width, trees.iter().map(Vec::len).sum());

    (trees, width, height)
}

#[test]
fn test_day_8_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(21, output);
}

#[test]
fn test_day_8_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1779, output);
}

#[test]
fn test_day_8_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(8, output);
}

#[test]
fn test_day_8_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(172224, output);
}

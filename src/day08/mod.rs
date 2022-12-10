use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 8;

    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        // TODO: could also take a transposed copy? probably slower though
        let (trees, stride) = parse_input_contiguous(input);

        let outer_ring_size = (stride - 1) * 4;

        // iterate over inner trees
        let mut highest_tree_vertical = (&trees[0..stride]).to_vec();
        let mut visible_trees = 0;
        for y in 1..(stride - 1) {
            let mut highest_tree_horizontal = trees[y * stride + 0];
            let mut bigger_tree_to_the_right = (0usize, 0u8);
            for x in 1..(stride - 1) {
                let current_tree = trees[y * stride + x];

                let tree_covered_left = current_tree <= highest_tree_horizontal;
                highest_tree_horizontal = highest_tree_horizontal.max(current_tree);

                let tree_covered_top = current_tree <= highest_tree_vertical[x];
                highest_tree_vertical[x] = highest_tree_vertical[x].max(current_tree);

                // check LOS left and up
                if !tree_covered_left || !tree_covered_top {
                    visible_trees += 1;
                    continue;
                }

                // check LOS right
                let mut tree_covered_right =
                    bigger_tree_to_the_right.0 > x && bigger_tree_to_the_right.1 >= current_tree;

                // if we don't have a known higher number to the right, loop through them to find
                // the first larger number and cache it.
                if !tree_covered_right {
                    bigger_tree_to_the_right.1 = 0;

                    for tree_x_index in (x.max(bigger_tree_to_the_right.0) + 1)..stride {
                        let tree_to_compare = trees[(y * stride) + tree_x_index];
                        if tree_to_compare >= current_tree {
                            // we found a tree that covers us
                            tree_covered_right = true;
                            bigger_tree_to_the_right.0 = tree_x_index;
                            bigger_tree_to_the_right.1 = tree_to_compare;

                            // breaking here means we look for the first number that is larger, rather than the largest number in general
                            // it turns out to be faster to do this.
                            // if we did want to look for the largest number in general, we'd need to add that check above
                            break;
                        }
                    }
                }
                if !tree_covered_right {
                    visible_trees += 1;
                    continue;
                }

                // check LOS down
                // we don't do the same optimization here because the additional complexity makes
                // it slower.
                let mut tree_covered = false;
                for y_check in (y + 1)..stride {
                    if current_tree <= trees[y_check * stride + x] {
                        // tree is covered
                        tree_covered = true;
                        break;
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

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        // let (trees, width, height) = parse_input(input);
        let (trees, stride) = parse_input_contiguous(input);

        let mut highest_score = 0;

        // outside trees will always be 0 because of the multiplication
        for y in 1..(stride - 1) {
            for x in 1..(stride - 1) {
                let start_tree = trees[y * stride + x];
                // println!("Current tree: {start_tree}");
                let mut score = 1;

                // left
                let mut score_calc = 0;
                for x_check in (0..x).rev() {
                    let tree = trees[y * stride + x_check];

                    score_calc += 1;

                    if tree >= start_tree {
                        break;
                    }
                }
                // println!("{x},{y} score left: {score_calc}");
                score *= score_calc;

                // right
                let mut score_calc = 0;
                for x_check in (x + 1)..stride {
                    let tree = trees[y * stride + x_check];

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
                    let tree = trees[y_check * stride + x];

                    score_calc += 1;

                    if tree >= start_tree {
                        break;
                    }
                }
                // println!("{x},{y} score up:   {score_calc}");
                score *= score_calc;

                // down
                let mut score_calc = 0;
                for y_check in (y + 1)..stride {
                    let tree = trees[y_check * stride + x];
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

fn parse_input_contiguous(input: &str) -> (Vec<u8>, usize) {
    let bytes = input.as_bytes();
    let stride = bytes.iter().position(|&b| b == b'\n').unwrap();
    let mut trees = vec![0; stride * stride]; // TODO: assuming square patch here
    let mut bytes_index = 0;
    for line in 0..stride {
        let start_line = line * stride;
        for vec_index in 0..stride {
            trees[start_line + vec_index] = bytes[bytes_index] - b'0';
            bytes_index += 1;
        }

        bytes_index += 1; // skip newline
    }

    (trees, stride)
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(21, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1779, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(8, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(172224, output);
}

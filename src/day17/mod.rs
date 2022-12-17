use ringbuffer::{AllocRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 17;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input: Vec<_> = input.as_bytes().iter().map(|b| *b == b'>').collect();
        let mut chamber: Vec<[bool; 7]> = vec![];

        let mut highest_y = -1isize;
        let mut input_index = 0;
        for i in 0..2023 {
            // iteration
            let block = BlockType::TYPES[i % 5];
            let block_height = block.get_height();
            let mut x = 2usize;
            let mut y = (highest_y + 3) as usize + block_height;

            while chamber.len() <= y {
                chamber.push([false; 7]);
            }

            debug_assert!(
                !block.collides(x as isize, y as isize, &chamber),
                "newly spawned block should not collide"
            );

            loop {
                // move horizontal
                let movement = input[input_index % input.len()];
                input_index += 1;

                let new_x = if movement {
                    x as isize + 1
                } else {
                    x as isize - 1
                };
                if !block.collides(new_x, y as isize, &chamber) {
                    x = new_x as usize;
                } else {
                }

                // drop down
                if !block.collides(x as isize, y as isize - 1, &chamber) {
                    y = y - 1;
                } else {
                    // could not drop down
                    // write the block to the chamber
                    for (tile_x, tile_y) in block.get_tiles() {
                        // let tile_y = block_height - tile_y;
                        chamber[y - *tile_y as usize][x + *tile_x as usize] = true;
                    }
                    highest_y = highest_y.max(y as isize);
                    break;
                }
            }
        }

        highest_y as usize - 1
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        const REQUIRED_ITERATIONS: usize = 1_000_000_000_000;

        let input: Vec<_> = input.as_bytes().iter().map(|b| *b == b'>').collect();
        let mut chamber: Vec<[bool; 7]> = vec![];

        let max_period_length: usize = input.len(); // the max length a period probably is. could be more
        let ring_buffer_size = (max_period_length * 2).next_power_of_two();

        let mut found_y_addition = None;
        let mut history = AllocRingBuffer::<(usize, usize)>::with_capacity(ring_buffer_size);

        let mut highest_y = -1isize; // highest y value that is in use
        let mut input_index = 0;

        let mut iteration = 0;
        while iteration < REQUIRED_ITERATIONS {
            if found_y_addition.is_none() {
                // push the current "state" so we can detect loops
                history.push(((highest_y + 1) as usize, input_index));

                // check if we hit looping point
                for period in 5..=(ring_buffer_size / 2) {
                    if iteration != 0 && history.len() >= (period * 2) {
                        let mut all_match = true;
                        for period_index in 0..period {
                            let one_index = period_index as isize;
                            let other_index = (period + period_index) as isize;
                            let one = history.get(-1 - one_index);
                            let other = history.get(-1 - other_index);

                            match (one, other) {
                                (Some((_y1, input_index1)), Some((_y2, input_index2))) => {
                                    if input_index1 == input_index2 {
                                        continue;
                                    }
                                }
                                _ => (),
                            }

                            all_match = false;
                            break;
                        }

                        if all_match {
                            let inc = history.get(-1).unwrap().0
                                - history.get(-1 - (period as isize)).unwrap().0;
                            let periods_to_skip = (REQUIRED_ITERATIONS - iteration) / period;

                            iteration += period * periods_to_skip;
                            let y_addition = inc * periods_to_skip;
                            found_y_addition = Some(y_addition);
                            break;
                        }
                    }
                }
            }

            // iteration
            let block = BlockType::TYPES[iteration % 5];
            let block_height = block.get_height();
            let mut x = 2usize;
            let mut y = (highest_y + 3) as usize + block_height;

            while chamber.len() <= y {
                chamber.push([false; 7]);
            }

            debug_assert!(
                !block.collides(x as isize, y as isize, &chamber),
                "newly spawned block should not collide"
            );

            loop {
                // move horizontal
                let movement = input[input_index % input.len()];
                input_index += 1;
                input_index %= input.len();

                let new_x = if movement {
                    x as isize + 1
                } else {
                    x as isize - 1
                };
                if !block.collides(new_x, y as isize, &chamber) {
                    x = new_x as usize;
                } else {
                }

                // drop down
                if !block.collides(x as isize, y as isize - 1, &chamber) {
                    y = y - 1;
                } else {
                    // could not drop down
                    // write the block to the chamber
                    for (tile_x, tile_y) in block.get_tiles() {
                        // let tile_y = block_height - tile_y;
                        chamber[y - *tile_y as usize][x + *tile_x as usize] = true;
                    }
                    highest_y = highest_y.max(y as isize);
                    break;
                }
            }

            iteration += 1;
        }

        highest_y as usize - 1 + found_y_addition.unwrap() + 2
    }
}

#[derive(Clone, Copy)]
enum BlockType {
    Minus,
    Plus,
    Corner,
    Line,
    Block,
}

impl BlockType {
    pub const TYPES: [BlockType; 5] = [
        BlockType::Minus,
        BlockType::Plus,
        BlockType::Corner,
        BlockType::Line,
        BlockType::Block,
    ];

    pub const fn get_tiles(&self) -> &[(isize, isize)] {
        match self {
            BlockType::Minus => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            BlockType::Plus => &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            BlockType::Corner => &[(2, 0), (2, 1), (2, 2), (0, 2), (1, 2)],
            BlockType::Line => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            BlockType::Block => &[(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }

    pub fn get_height(&self) -> usize {
        match self {
            BlockType::Minus => 1,
            BlockType::Plus => 3,
            BlockType::Corner => 3,
            BlockType::Line => 4,
            BlockType::Block => 2,
        }
    }

    pub fn collides(&self, x: isize, y: isize, chamber: &[[bool; 7]]) -> bool {
        self.get_tiles().iter().any(|(tile_x, tile_y)| {
            let new_y = y - tile_y;
            let new_x = x + tile_x;
            let collides = !(0..7).contains(&new_x)
                || !(0..).contains(&new_y)
                || chamber[new_y as usize][new_x as usize];
            collides
        })
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(3068, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(3141, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(1514285714288, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(1561739130391, output);
}

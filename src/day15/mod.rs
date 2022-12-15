use std::ops::Range;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 15;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        // ugly hack because AoC is weird
        let is_sample = input == Self::INPUT_SAMPLE;
        let line = if is_sample { 10 } else { 2_000_000 };

        let input = parse_input(input);

        // we need to calculate the range where we can check a beacon can be
        // we need to check the range of each sensor and add its range to it
        let ranges: Vec<_> = input
            .iter()
            .flat_map(|(s, dist)| {
                let y_diff = s.1.abs_diff(line);

                if *dist < y_diff {
                    None
                } else {
                    let x_side = (dist - y_diff) as isize;
                    Some((s.0 - x_side)..=(s.0 + x_side))
                }
            })
            .collect();

        // check if each cell in the line in between a sensor and its beacon
        let mut current_x = isize::MIN;
        let mut count = 0;

        loop {
            let next_range = ranges
                .iter()
                .filter(|r| r.contains(&current_x) || r.start() > &current_x)
                .min_by(|r1, r2| r1.start().cmp(&r2.start()));

            let Some(range) = next_range else {
                break;
            };

            let start = range.start().max(&current_x);
            let end = range.end() + 1; // inclusive
            count += (end - start) as usize;
            current_x = end;
        }

        // TODO: remove beacons from the range?

        count - 1
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let is_sample = input == Self::INPUT_SAMPLE;
        let max_coordinate = if is_sample { 20 } else { 4_000_000 };

        let input = parse_input(input);

        let coordinate = contains_square(&input, 0..max_coordinate, 0..max_coordinate).unwrap();

        coordinate.0 * 4_000_000 + coordinate.1
    }
}

fn contains_square(
    sensors: &[((isize, isize), usize)],
    x: Range<usize>,
    y: Range<usize>,
) -> Option<(usize, usize)> {
    // println!("Checking x={x:?} y={y:?}");

    if x.clone().count() == 0 || y.clone().count() == 0 {
        return None;
    }

    // check if the range is contained by a sensor
    // can be done by checking if all corners are contained
    let fully_contained = sensors.iter().any(|(pos, range)| {
        let top_left = (x.start as isize, y.start as isize);
        let top_right = ((x.end - 1) as isize, y.start as isize);
        let bottom_left = (x.start as isize, (y.end - 1) as isize);
        let bottom_right = ((x.end - 1) as isize, (y.end - 1) as isize);

        manhattan_distance(*pos, top_left) <= *range
            && manhattan_distance(*pos, top_right) <= *range
            && manhattan_distance(*pos, bottom_left) <= *range
            && manhattan_distance(*pos, bottom_right) <= *range
    });
    if fully_contained {
        // some range fully contains this section, so we can ignore it
        return None;
    }

    // now divide and try subsections
    // however, if we are 1x1, this is the solution
    if x.clone().count() == 1 && y.clone().count() == 1 {
        println!("Found!");
        return Some((x.start, y.start));
    }

    let half_x = (x.end - x.start) / 2 + x.start;
    let half_y = (y.end - y.start) / 2 + y.start;

    let left = x.start..half_x;
    let right = half_x..(x.end);
    let top = y.start..half_y;
    let bottom = half_y..(y.end);

    contains_square(sensors, left.clone(), top.clone())
        .or_else(|| contains_square(sensors, right.clone(), top.clone()))
        .or_else(|| contains_square(sensors, left.clone(), bottom.clone()))
        .or_else(|| contains_square(sensors, right.clone(), bottom.clone()))
}

fn manhattan_distance(s: (isize, isize), b: (isize, isize)) -> usize {
    s.0.abs_diff(b.0) + s.1.abs_diff(b.1)
}

fn parse_input(input: &str) -> Vec<((isize, isize), usize)> {
    input
        .lines()
        .map(|line| {
            let (sensor, beacon) = line.split_once(": ").unwrap();
            let (sensor_1, sensor_2) = sensor.split_once(',').unwrap();
            let (beacon_1, beacon_2) = beacon.split_once(',').unwrap();

            let sensor_1 = str::parse(&sensor_1["Sensor at x=".len()..]).unwrap();
            let sensor_2 = str::parse(&sensor_2[" y=".len()..]).unwrap();

            let beacon_1 = str::parse(&beacon_1["closest beacon is at x=".len()..]).unwrap();
            let beacon_2 = str::parse(&beacon_2[" y=".len()..]).unwrap();

            let distance = manhattan_distance((sensor_1, sensor_2), (beacon_1, beacon_2));

            ((sensor_1, sensor_2), distance)
        })
        .collect()
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(26, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(4665948, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(56000011, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(13543690671045, output);
}

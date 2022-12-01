fn main() {
    let input = include_str!("input.txt");
    let output = do_day_1_silver(input);
    println!("Silver: {output}");
    let output = do_day_1_gold(input);
    println!("gold: {output}");
}

fn do_day_1_silver(data: &str) -> u32 {
    let mut lines = data.trim().lines().peekable();
    let mut max = 0;

    while lines.peek().is_some() {
        let sum = lines
            .by_ref()
            .take_while(|&l| !l.is_empty())
            .flat_map(|l| l.parse::<u32>())
            .sum();

        max = max.max(sum);
    }

    max
}

fn do_day_1_gold(data: &str) -> u32 {
    let mut lines = data.trim().lines().peekable();

    // note: order is from least to most
    let mut max = [0, 0, 0];

    while lines.peek().is_some() {
        let sum = lines
            .by_ref()
            .take_while(|&l| !l.is_empty())
            .flat_map(|l| l.parse::<u32>())
            .sum();

        if sum <= max[0] {
            continue;
        }

        if sum <= max[1] {
            max[0] = sum;
            continue;
        }

        if sum <= max[2] {
            max[0] = max[1];
            max[1] = sum;
            continue;
        }

        max[0] = max[1];
        max[1] = max[2];
        max[2] = sum;
    }

    max.into_iter().sum()
}

#[test]
fn test_day_1_silver_sample() {
    let input = include_str!("sample.txt");
    let output = do_day_1_silver(input);
    assert_eq!(24000, output);
}

#[test]
fn test_day_1_silver_real() {
    let input = include_str!("input.txt");
    let output = do_day_1_silver(input);
    assert_eq!(67633, output);
}

#[test]
fn test_day_1_gold_sample() {
    let input = include_str!("sample.txt");
    let output = do_day_1_gold(input);
    assert_eq!(45000, output);
}

#[test]
fn test_day_1_gold_real() {
    let input = include_str!("input.txt");
    let output = do_day_1_gold(input);
    assert_eq!(199628, output);
}

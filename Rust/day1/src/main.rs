use std::error::Error;
use std::str::FromStr;

fn part1(measurements: &[usize]) -> usize {
    let mut counter = 0;
    for measurement in measurements.windows(2) {
        if measurement[0] < measurement[1] {
            counter += 1;
        }
    }
    counter
}

fn part2(measurements: &[usize]) -> usize {
    let mut previous_sum = None;
    let mut counter = 0;
    for measurement in measurements.windows(3) {
        let actual_sum = measurement[0] + measurement[1] + measurement[2];
        if let Some(prev) = previous_sum {
            if prev < actual_sum {
                counter += 1;
            }
        }
        previous_sum = Some(actual_sum);
    }
    counter
}

fn main() {
    let input = include_str!("../input.txt");
    let measurements: Vec<_> = input
        .split_whitespace()
        .filter_map(|w| usize::from_str(w).ok())
        .collect();

    let result1 = part1(&measurements);
    let result2 = part2(&measurements);
    println!("result1: {}", result1);
    println!("result2: {}", result2);
}

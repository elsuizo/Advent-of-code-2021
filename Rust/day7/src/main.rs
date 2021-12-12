// use std::ops::RangeInclusive;
use std::str::FromStr;

fn median(input: &[usize]) -> usize {
    let len = input.len();
    if len % 2 == 0 {
        (input[len / 2] + input[len / 2 - 1]) / 2
    } else {
        input[(len / 2) - 1]
    }
}

fn part1(median: usize, positions: &[usize]) -> usize {
    let mut result: i32 = 0;
    for position in positions {
        let number = (*position as i32 - median as i32).abs();
        result += number;
    }
    result as usize
}

/// cumsum implementation
///
/// input: `end`: `usize`
///
/// output: `usize` the value of perform a cumulative sum from 0 to end
fn cumsum(end: usize) -> usize {
    (1..=end).fold(0, |sum, item| sum + item)
}

fn move_cost(initial: i32, end: i32) -> usize {
    cumsum((initial - end).abs() as usize)
}

fn part2(positions: &[usize]) -> usize {}

// TODO(elsuizo:2021-12-11): esta manera de parsear el input tiene el problema de que no lee el
// ultimo numbero y hay que agregarle una coma al final ...
fn main() {
    let input = include_str!("../input_small.txt");
    let mut horizontal_positions: Vec<_> = input
        .split(',')
        .filter_map(|w| usize::from_str(w).ok())
        .collect();

    horizontal_positions.sort();
    let median = median(&horizontal_positions);
    let result1 = part1(median, &horizontal_positions);
    println!("result1: {}", result1);

    let result2 = part2(&horizontal_positions);
    println!("result2: {}", result2);
}

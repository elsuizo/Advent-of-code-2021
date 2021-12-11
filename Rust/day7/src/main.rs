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

fn part2(optimal_position: usize, positions: &[usize]) -> usize {}

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
}

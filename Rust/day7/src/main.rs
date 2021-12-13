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

// TODO(elsuizo:2021-12-12): esto supone que las posiciones estan ordenadas deberiamos ordenarlas
// en cada funcion capaz...
fn part2(positions: &[usize]) -> Option<usize> {
    let mut optimal_cost = i32::MAX;
    for optimal_candidate in 0..*positions.last()? {
        let mut total_fuel = 0;
        for position in positions {
            total_fuel += move_cost(optimal_candidate as i32, *position as i32);
        }
        if total_fuel < optimal_cost as usize {
            optimal_cost = total_fuel as i32;
        }
    }
    Some(optimal_cost as usize)
}

// TODO(elsuizo:2021-12-11): esta manera de parsear el input tiene el problema de que no lee el
// ultimo numero y hay que agregarle una coma al final ...
fn main() {
    let input = include_str!("../input.txt");
    let mut horizontal_positions: Vec<_> = input
        .split(',')
        .filter_map(|w| usize::from_str(w).ok())
        .collect();

    horizontal_positions.sort();
    let median = median(&horizontal_positions);
    let result1 = part1(median, &horizontal_positions);
    println!("result1: {}", result1);

    if let Some(result2) = part2(&horizontal_positions) {
        println!("result2: {}", result2);
    }
}

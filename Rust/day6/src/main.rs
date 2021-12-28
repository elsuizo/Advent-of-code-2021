use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct LanternFish {
    internal_clock: usize,
}

impl LanternFish {
    fn new(internal_clock: usize) -> Self {
        Self { internal_clock }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input_small.txt");
    let initial_states: Vec<_> = input
        .split(',')
        .filter_map(|w| usize::from_str(w).ok())
        .map(|n| LanternFish::new(n))
        .collect();

    println!("initial_states: {:?}", initial_states);

    Ok(())
}

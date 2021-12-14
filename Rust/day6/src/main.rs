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

    fn decrement_internal_clock(&mut self) -> bool {
        if self.internal_clock == 6 {
            self.internal_clock = 0;
            return true;
        }
        self.internal_clock -= 1;
        return false;
    }
}

fn simulate_system(initial_fishs: &mut [LanternFish], days: usize) {
    for day in 0..days {
        for fish in initial_fishs.iter_mut() {
            if fish.decrement_internal_clock() {
                initial_fishs += LanternFish::new(8);
            }
        }
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

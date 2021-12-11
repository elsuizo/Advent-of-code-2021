use std::cell::Cell;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct LanternFish {
    timer: usize,
}

impl LanternFish {
    fn new(timer: usize) -> Self {
        Self { timer }
    }

    // TODO(elsuizo:2021-12-06): hacer que estas variables hardcodeadas sean const
    fn decrement_timer(&mut self) {
        if self.timer == 0 {
            self.timer = 6;
        } else {
            self.timer -= 1;
        }
    }
}

struct System {
    initial_fishs: Vec<LanternFish>,
}

impl System {
    fn new(fishs: Vec<LanternFish>) -> Self {
        Self { fishs }
    }

    fn add_new_fish(&mut self) {
        self.fishs.push(LanternFish::new(8));
    }

    fn simulate(&mut self, days: usize) {
        for _ in 0..days {
            for mut fish in self.fishs.iter_mut() {
                if fish.timer == 0 {
                    self.add_new_fish();
                }
                fish.decrement_timer();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input_small.txt");
    let initial_states: Vec<_> = input
        .split(',')
        .filter_map(|w| usize::from_str(w).ok())
        .collect();

    let mut fishs = Vec::new();
    for state in initial_states {
        fishs.push(LanternFish::new(state));
    }

    Ok(())
}

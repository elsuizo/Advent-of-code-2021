use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug)]
struct Position {
    horizontal: usize,
    depth: usize,
}

#[derive(Debug, Default)]
struct Position2 {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

impl Position2 {
    fn update(&mut self, command: Command) {
        use Command::*;
        match command {
            Forward(units) => {
                self.horizontal += units;
                self.depth += self.aim * units;
            }
            Down(units) => self.aim += units,
            Up(units) => self.aim -= units,
        }
    }
}

impl Position {
    fn init() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }

    fn update(&mut self, command: Command) {
        use Command::*;
        match command {
            Forward(units) => self.horizontal += units,
            Down(units) => self.depth += units,
            Up(units) => self.depth -= units,
        }
    }
}

fn parse_input(line: &str) -> Result<Command, Box<dyn Error>> {
    use Command::*;
    let v: Vec<&str> = line.split_whitespace().collect();
    match v[0] {
        "forward" => Ok(Forward(usize::from_str(v[1])?)),
        "down" => Ok(Down(usize::from_str(v[1])?)),
        "up" => Ok(Up(usize::from_str(v[1])?)),
        _ => panic!("erorr unknown command!!!"),
    }
}

fn part1(commands: &[Command], position: &mut Position) -> usize {
    for command in commands {
        position.update(*command);
    }
    position.horizontal * position.depth
}

fn part2(commands: &[Command], position: &mut Position2) -> usize {
    for command in commands {
        position.update(*command);
    }
    position.horizontal * position.depth
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    let mut commands = Vec::new();
    let mut position = Position::init();
    let mut position2 = Position2::default();
    for line in input.lines() {
        commands.push(parse_input(line)?);
    }

    let result1 = part1(&commands, &mut position);
    let result2 = part2(&commands, &mut position2);
    println!("result1: {}", result1);
    println!("result2: {}", result2);

    Ok(())
}

use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Bin {
    data: Vec<bool>,
}

impl Bin {
    fn new(data: Vec<bool>) -> Self {
        Self { data }
    }

    fn to_decimal(&self) -> usize {
        let mut result: u8 = 0;
        self.data.iter().for_each(|&bit| {
            result <<= 1;
            result ^= bit as u8;
        });
        result as usize
    }
}

impl FromStr for Bin {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Vec::new();
        for bit in s.chars() {
            result.push(bit.to_digit(10).unwrap() != 0)
        }
        Ok(Bin::new(result))
    }
}

#[derive(Debug)]
struct Report {
    bin: Vec<Bin>,
}

impl Report {
    fn new(bin: Vec<Bin>) -> Self {
        Self { bin }
    }

    fn count_true(&self, position: usize) -> usize {
        let mut counter = 0;
        for index in 0..self.bin.len() {
            if self.bin[index].data[position] {
                counter += 1;
            }
        }
        counter
    }

    fn count_false(&self, position: usize) -> usize {
        self.bin.len() - self.count_true(position)
    }

    fn generate_gamma_rate(&self) -> Bin {
        let mut result = Vec::new();
        let len = self.bin[0].data.len();
        for index in 0..len {
            let trues = self.count_true(index);
            let falses = self.count_false(index);
            if trues > falses {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        Bin::new(result)
    }

    fn generate_epsilon_rate(&self) -> Bin {
        let gamma = self.generate_gamma_rate();
        let mut result_vec = Vec::new();
        for element in gamma.data {
            result_vec.push(!element);
        }
        Bin::new(result_vec)
    }

    fn power_consumption(&self) -> usize {
        self.generate_gamma_rate().to_decimal() * self.generate_epsilon_rate().to_decimal()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input_small.txt");
    let mut bins = Vec::new();
    for line in input.lines() {
        bins.push(Bin::from_str(line)?);
    }
    let report = Report::new(bins);

    println!("power_consumption: {:}", report.power_consumption());

    Ok(())
}

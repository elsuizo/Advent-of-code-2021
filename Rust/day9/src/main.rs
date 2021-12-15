use std::error::Error;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Default, PartialOrd)]
struct Height {
    element: (u8, bool),
}

impl Height {
    fn new(element: u8) -> Self {
        Self {
            element: (element, false),
        }
    }
}

impl PartialEq for Height {
    fn eq(&self, other: &Self) -> bool {
        self.element.0 == other.element.0
    }
}

#[derive(Debug, Default)]
struct HeightMap {
    size: (usize, usize),
    heights: Vec<Height>,
}

// TODO(elsuizo:2021-12-14): hay que hacer que sea generico y que utilice los rows y cols
impl HeightMap {
    fn init(rows: usize, cols: usize, input: &str) -> Self {
        let mut result = Self {
            size: (rows, cols),
            heights: Vec::with_capacity(rows * cols),
        };

        for line in input.lines() {
            for num in line.chars().flat_map(|n| n.to_digit(10)) {
                result.heights.push(Height::new(num as u8));
            }
        }
        result
    }

    fn cols(&self) -> usize {
        self.size.1
    }

    fn rows(&self) -> usize {
        self.size.0
    }

    fn compute_adjacent_locations(&mut self) {
        let cols = self.cols();
        let rows = self.rows();
        for i in 0..rows {
            for j in 0..cols {
                match (i, j) {
                    (0, 0) => {
                        if self[(i, j)] < self[(i, j + 1)] && self[(i, j)] < self[(i + 1, j)] {
                            self[(i, j)].element.1 = true;
                        }
                    }
                    (0, 9) => {
                        if self[(i, j)] < self[(i, j - 1)] && self[(i, j)] < self[(i + 1, j)] {
                            self[(i, j)].element.1 = true;
                        }
                    }
                    (4, 0) => {
                        if self[(i, j)] < self[(i - 1, j)] && self[(i, j)] < self[(i, j + 1)] {
                            self[(i, j)].element.1 = true;
                        }
                    }
                    (4, 9) => {
                        if self[(i, j)] < self[(i, j - 1)] && self[(i, j)] < self[(i - 1, j)] {
                            self[(i, j)].element.1 = true;
                        }
                    }
                    (0, 1..=8) => {
                        if self[(i, j)] < self[(i, j - 1)]
                            && self[(i, j)] < self[(i, j + 1)]
                            && self[(i, j)] < self[(i + 1, j)]
                        {
                            self[(i, j)].element.1 = true;
                        }
                    }
                    (4, 1..=8) => {
                        if self[(i, j)] < self[(i, j - 1)]
                            && self[(i, j)] < self[(i, j + 1)]
                            && self[(i, j)] < self[(i - 1, j)]
                        {
                            self[(i, j)].element.1 = true;
                        }
                    }
                    (1..=4, 0) => {
                        if self[(i, j)] < self[(i - 1, j)]
                            && self[(i, j)] < self[(i, j + 1)]
                            && self[(i, j)] < self[(i + 1, j)]
                        {
                            self[(i, j)].element.1 = true;
                        }
                    }
                    (1..=4, 9) => {
                        if self[(i, j)] < self[(i - 1, j)]
                            && self[(i, j)] < self[(i, j - 1)]
                            && self[(i, j)] < self[(i + 1, j)]
                        {
                            self[(i, j)].element.1 = true;
                        }
                    }
                    (_, _) => {
                        if self[(i, j)] < self[(i, j - 1)]
                            && self[(i, j)] < self[(i, j + 1)]
                            && self[(i, j)] < self[(i - 1, j)]
                            && self[(i, j)] < self[(i + 1, j)]
                        {
                            self[(i, j)].element.1 = true;
                        }
                    }
                }
            }
        }
    }

    fn minimums(&self) {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if self[(i, j)].element.1 == true {
                    println!("minimum: {} @ [({}, {})]", self[(i, j)].element.0, i, j);
                }
            }
        }
    }

    fn compute_risk_level(&self) -> usize {
        let mut result = 0;
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if self[(i, j)].element.1 == true {
                    println!("minimum: {} @ [({}, {})]", self[(i, j)].element.0, i, j);
                    result += 1 + self[(i, j)].element.0 as usize;
                }
            }
        }
        result
    }
}

//-------------------------------------------------------------------------
//                        index impl
//-------------------------------------------------------------------------
impl Index<(usize, usize)> for HeightMap {
    type Output = Height;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let (_, h) = self.size;
        &self.heights[x * h + y]
    }
}

impl IndexMut<(usize, usize)> for HeightMap {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let (_, h) = self.size;
        &mut self.heights[x * h + y]
    }
}

//-------------------------------------------------------------------------
//                        main
//-------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input_small.txt");
    let mut height_map = HeightMap::init(5, 10, input);

    height_map.compute_adjacent_locations();

    // height_map.minimums();
    let result1 = height_map.compute_risk_level();
    Ok(())
}

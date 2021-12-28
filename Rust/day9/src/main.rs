use std::error::Error;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Default, PartialOrd, Copy)]
pub struct Height {
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

#[derive(Debug, PartialEq)]
pub struct HeightMap<const M: usize, const N: usize>([[Height; N]; M]);

//-------------------------------------------------------------------------
//                        index impl
//-------------------------------------------------------------------------
impl<const M: usize, const N: usize> Index<(usize, usize)> for HeightMap<M, N> {
    type Output = Height;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[x][y]
    }
}

impl<const M: usize, const N: usize> IndexMut<(usize, usize)> for HeightMap<M, N> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[x][y]
    }
}

impl<const M: usize, const N: usize> HeightMap<M, N> {
    fn new(data_input: [[Height; N]; M]) -> Self {
        Self(data_input)
    }

    fn init(input: &str) -> Self {
        let mut result = HeightMap::new([[Height::default(); N]; M]);
        let mut aux = Vec::new();
        for line in input.lines() {
            for num in line.chars().flat_map(|n| n.to_digit(10)) {
                aux.push(Height::new(num as u8));
            }
        }
        for i in (0..M).rev() {
            for j in (0..N).rev() {
                result[(i, j)] = aux.pop().unwrap();
            }
        }
        result
    }

    fn check_neighbours(&self, x: usize, y: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let idx = y * N * x;
        if x > 0 {
            result.push(idx - 1);
        }
        if x <
    }
}

//-------------------------------------------------------------------------
//                        main
//-------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input_small.txt");
    let mut height_map = HeightMap::<5, 10>::init(input);

    println!("height_map: {:?}", height_map);
    // height_map.compute_adjacent_locations();

    // height_map.minimums();
    // let result1 = height_map.compute_risk_level();
    Ok(())
}

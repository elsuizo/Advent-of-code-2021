use std::error::Error;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
enum AdjacencyType {
    Corner([u8; 2]),
    Normal([u8; 4]),
    Edge([u8; 3]),
}

#[derive(Debug, Clone, Default)]
struct Height {
    element: u8,
    adjacency_type: AdjacencyType,
}

impl Height {
    fn new(element: u8, adjacency_type: AdjacencyType) -> Self {
        Self {
            element,
            adjacency_type,
        }
    }
}

#[derive(Debug, Default)]
struct HeightMap {
    size: (usize, usize),
    heights: Vec<Height>,
}

impl HeightMap {
    fn init(rows: usize, cols: usize, input: &str) -> Self {
        let mut result = Self {
            size: (rows, cols),
            heights: vec![Height::default(); rows * cols],
        };

        for line in input.lines() {
            for i in 0..rows {
                for j in 0..cols {
                    for num in line.chars().flat_map(|n| n.to_digit(10)) {
                        println!("({}, {}): {:?}", i, j, result[(i, j)]);
                        // match (i, j) {
                        //     (0, col @ 1..=8) => {
                        //         result[(i, col)] =
                        //             Height::new(num as u8, AdjacencyType::Edge([0u8; 3]))
                        //     }
                        //     (5, col @ 1..=8) => {
                        //         result[(i, col)] =
                        //             Height::new(num as u8, AdjacencyType::Edge([0u8; 3]))
                        //     }
                        //     (row @ 1..=3, 0) => {
                        //         result[(row, j)] =
                        //             Height::new(num as u8, AdjacencyType::Edge([0u8; 3]))
                        //     }
                        //     (row @ 1..=3, 10) => {
                        //         result[(row, j)] =
                        //             Height::new(num as u8, AdjacencyType::Edge([0u8; 3]))
                        //     }
                        //     // (0, 0) | (0, 9) | (4, 0) | (4, 9) => {
                        //     //     result[(i, j)] =
                        //     //         Height::new(num as u8, AdjacencyType::Corner([0u8; 2]))
                        //     // }
                        //     (_, _) => {
                        //         result[(i, j)] =
                        //             Height::new(num as u8, AdjacencyType::Normal([0u8; 4]))
                        //     }
                        // }
                    }
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
    println!("height_map: {:?}", height_map);

    Ok(())
}

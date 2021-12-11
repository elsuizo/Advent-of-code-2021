use std::error::Error;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
enum AdjacencyType {
    Corner([u8; 2]),
    Normal([u8; 4]),
    Edge([u8; 3]),
}

#[derive(Debug)]
struct Height {
    element: u8,
    adjacency_type: Option<AdjacencyType>,
}

impl Height {
    fn new(element: u8) -> Self {
        Self {
            element,
            adjacency_type: None,
        }
    }
}

#[derive(Debug, Default)]
struct HeightMap {
    size: (usize, usize),
    heights: Vec<Height>,
}

impl HeightMap {
    fn init(rows: usize, cols: usize) -> Self {
        Self {
            size: (rows, cols),
            heights: Vec::with_capacity(rows * cols),
        }
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
    let mut height_map = HeightMap::init(5, 10);
    for line in input.lines() {
        for (index, num) in line.chars().flat_map(|n| n.to_digit(10)).enumerate() {
            let height = Height::new(num as u8);
            height_map.heights.push(height);
        }
    }
    println!("height_map: {:?}", height_map);

    Ok(())
}

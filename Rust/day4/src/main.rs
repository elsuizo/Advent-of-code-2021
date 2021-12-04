use std::collections::HashSet;
use std::error::Error;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

// TODO(elsuizo:2021-12-04):
// - [  ] hacer las funciones para la parte 1 y la parte2
// - [  ] mejorar la parte del chequeo de las columnas y las filas

//-------------------------------------------------------------------------
//                        Board definition
//-------------------------------------------------------------------------
#[derive(Debug, Default)]
struct Board {
    elements: Vec<(usize, bool)>,
}

impl Board {
    fn new() -> Self {
        Self {
            elements: Vec::with_capacity(5 * 5),
        }
    }

    fn set_number(&mut self, number: usize) {
        for i in 0..5 {
            for j in 0..5 {
                if self[(i, j)] == (number, false) {
                    self[(i, j)] = (number, true)
                }
            }
        }
    }

    fn check_cols(&self) -> bool {
        let mut counter;
        for i in 0..5 {
            counter = 0;
            for j in 0..5 {
                if self[(i, j)].1 {
                    counter += 1;
                }
                if counter == 5 {
                    return true;
                }
            }
        }
        false
    }

    fn check_rows(&self) -> bool {
        let mut counter;
        for j in 0..5 {
            counter = 0;
            for i in 0..5 {
                if self[(i, j)].1 {
                    counter += 1;
                }
                if counter == 5 {
                    return true;
                }
            }
        }
        false
    }

    fn check_win(&self) -> bool {
        self.check_rows() || self.check_cols()
    }

    fn show_elements(&self) {
        for i in 0..5 {
            for j in 0..5 {
                println!("self[({}, {})]: {:?}", i, j, self[(i, j)]);
            }
        }
    }
}

//-------------------------------------------------------------------------
//                        index impl
//-------------------------------------------------------------------------
impl Index<(usize, usize)> for Board {
    type Output = (usize, bool);
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elements[index.1 * 5 + index.0]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.elements[index.1 * 5 + index.0]
    }
}

#[derive(Debug)]
struct Bingo {
    boards: Vec<Board>,
    numbers: Vec<usize>,
}

impl Bingo {
    fn new(boards: Vec<Board>, numbers: Vec<usize>) -> Self {
        Self { boards, numbers }
    }

    fn simulate(&mut self) -> (usize, usize) {
        for number in &self.numbers {
            for (board_number, board) in self.boards.iter_mut().enumerate() {
                board.set_number(*number);
                if board.check_win() {
                    return (board_number, *number);
                }
            }
        }
        (0, 0)
    }

    fn simulate2(&mut self) -> (usize, usize) {
        let mut hashset = HashSet::new();
        let len = self.boards.len();
        for number in &self.numbers {
            for (board_number, board) in self.boards.iter_mut().enumerate() {
                board.set_number(*number);
                if board.check_win() {
                    hashset.insert(board_number);
                    if hashset.len() == len {
                        return (board_number, *number);
                    }
                }
            }
        }
        (0, 0)
    }

    fn sum_unmarked(&self, board_number: usize) -> usize {
        let mut counter = 0;
        let board = &self.boards[board_number];
        for i in 0..5 {
            for j in 0..5 {
                if !board[(i, j)].1 {
                    counter += board[(i, j)].0;
                }
            }
        }
        counter
    }

    fn calculate_score(&mut self) -> usize {
        let (board_number, number) = self.simulate();
        let sum_unmarked = self.sum_unmarked(board_number);
        number * sum_unmarked
    }

    fn calculate_score2(&mut self) -> usize {
        let (board_number, number) = self.simulate2();
        let sum_unmarked = self.sum_unmarked(board_number);
        number * sum_unmarked
    }

    fn check_board(&self, number: usize) {
        println!("self.boards: {:?}", self.boards[number]);
    }
}

//-------------------------------------------------------------------------
//                        main
//-------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");

    let mut numbers = Vec::new();
    for line in input.lines().take(1) {
        numbers = line
            .split(',')
            .filter_map(|n| usize::from_str(n).ok())
            .collect();
    }

    let mut boards = Vec::new();
    for raw_input in input.split("\n\n").skip(1) {
        let mut board = Board::new();
        for raw_row in raw_input.split_whitespace() {
            let number = usize::from_str(raw_row)?;
            board.elements.push((number, false));
        }
        boards.push(board);
    }

    let mut bingo = Bingo::new(boards, numbers);
    let score = bingo.calculate_score2();
    println!("score: {:?}", score);

    Ok(())
}

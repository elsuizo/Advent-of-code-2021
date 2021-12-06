use std::collections::HashMap;
use std::error::Error;
//-------------------------------------------------------------------------
//                        Line
//-------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
enum LineType {
    Vertical,
    Horizontal,
    WithSlope,
}

#[derive(Debug)]
struct Line {
    p1: (usize, usize),
    p2: (usize, usize),
    line_type: LineType,
}

impl Line {
    fn new(p1: (usize, usize), p2: (usize, usize), line_type: LineType) -> Self {
        Self { p1, p2, line_type }
    }
}

use std::str::FromStr;

fn parse_pairs<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn vertical_or_horizontal((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> LineType {
    use std::cmp::Ordering::*;
    match (x1.cmp(&x2), y1.cmp(&y2)) {
        (Equal, _) => LineType::Vertical,
        (_, Equal) => LineType::Horizontal,
        (_, _) => LineType::WithSlope,
    }
}

/// generate all the discretes points from begin to end
fn generate_path(lines: &[Line]) -> HashMap<(usize, usize), usize> {
    let mut result = HashMap::new();
    for line in lines {
        let (x1, y1) = line.p1;
        let (x2, y2) = line.p2;
        match line.line_type {
            LineType::Vertical => {
                if y1 < y2 {
                    for y in y1..=y2 {
                        *result.entry((x1, y)).or_insert(0) += 1;
                    }
                } else {
                    for y in y2..=y1 {
                        *result.entry((x1, y)).or_insert(0) += 1;
                    }
                }
            }
            LineType::Horizontal => {
                if x1 < x2 {
                    for x in x1..=x2 {
                        *result.entry((x, y1)).or_insert(0) += 1;
                    }
                } else {
                    for x in x2..=x1 {
                        *result.entry((x, y1)).or_insert(0) += 1;
                    }
                }
            }
            // TODO(elsuizo:2021-12-05): no me gusta como quedo esto ...
            LineType::WithSlope => {
                use std::cmp::Ordering::*;
                match (x1.cmp(&x2), y1.cmp(&y2)) {
                    (Greater, Greater) => {
                        let mut x = x1;
                        let mut y = y1;
                        *result.entry((x1, y1)).or_insert(0) += 1;
                        while !(x == x2 && y == y2) {
                            x -= 1;
                            y -= 1;
                            *result.entry((x, y)).or_insert(0) += 1;
                        }
                    }
                    (Greater, Less) => {
                        let mut x = x1;
                        let mut y = y1;
                        *result.entry((x1, y1)).or_insert(0) += 1;
                        while !(x == x2 && y == y2) {
                            x -= 1;
                            y += 1;
                            *result.entry((x, y)).or_insert(0) += 1;
                        }
                    }
                    (Less, Greater) => {
                        let mut x = x1;
                        let mut y = y1;
                        *result.entry((x1, y1)).or_insert(0) += 1;
                        while !(x == x2 && y == y2) {
                            x += 1;
                            y -= 1;
                            *result.entry((x, y)).or_insert(0) += 1;
                        }
                    }
                    (Less, Less) => {
                        let mut x = x1;
                        let mut y = y1;
                        *result.entry((x1, y1)).or_insert(0) += 1;
                        while !(x == x2 && y == y2) {
                            x += 1;
                            y += 1;
                            *result.entry((x, y)).or_insert(0) += 1;
                        }
                    }
                    (_, _) => {}
                }
            }
        }
    }
    result
}
//-------------------------------------------------------------------------
//                       main
//-------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");

    let mut lines = Vec::new();
    for raw_line in input.lines() {
        let raw_points: Vec<_> = raw_line.split(" -> ").collect();
        let pair1 = parse_pairs::<usize>(raw_points[0], ',').expect("parse error!!!");
        let pair2 = parse_pairs::<usize>(raw_points[1], ',').expect("parse error!!!");
        let line_type = vertical_or_horizontal(pair1, pair2);
        lines.push(Line::new(pair1, pair2, line_type));
    }

    let map = generate_path(&lines);
    // println!("map: {:?}", map);
    let result1 = map.values().filter(|&&v| v >= 2).count();
    println!("result1: {}", result1);
    Ok(())
}

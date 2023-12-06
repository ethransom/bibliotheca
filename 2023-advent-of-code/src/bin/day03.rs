// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example03.txt");
const INPUT: &str = include_str!("input03.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let schematic = parse(input);

    let part_nos = collect_part_nos(&schematic);

    let mut gear_ratios = HashMap::<(usize, usize), Vec<usize>>::new();
    let mut part_no = String::new();
    let mut touches_gear: Option<(usize, usize)> = None;
    for (y, row) in schematic.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                ('0'..='9') => {
                    part_no.push(*cell);

                    neighbors(x, y, row.len(), schematic.len()).for_each(|(nx, ny)| {
                        let neighbor = schematic[ny][nx];
                        if neighbor == '*' {
                            touches_gear = Some((nx, ny));
                        }
                    });
                }
                _ => {
                    if let Some((gx, gy)) = touches_gear {
                        if !part_no.is_empty() {
                            gear_ratios
                                .entry((gx, gy))
                                .or_default()
                                .push(part_no.parse().unwrap());
                        }
                    }
                    part_no.clear();
                    touches_gear = None;
                }
            }
        }
        if let Some((gx, gy)) = touches_gear {
            if !part_no.is_empty() {
                gear_ratios
                    .entry((gx, gy))
                    .or_default()
                    .push(part_no.parse().unwrap());
            }
        }
        part_no.clear();
        touches_gear = None;
    }

    let gear_ratios_sum = gear_ratios
        .iter()
        .filter_map(|(_, part_nos)| {
            if part_nos.len() == 2 {
                Some(part_nos.iter().product::<usize>())
            } else {
                dbg!(part_nos);
                None
            }
        })
        .sum();

    (part_nos.iter().sum(), gear_ratios_sum)
}

struct Schematic {
    schematic: Vec<Vec<char>>,
}

impl Schematic {
    #[allow(dead_code)]

    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.schematic.get(y).and_then(|row| row.get(x)).copied()
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize, char)> + '_ {
        self.schematic
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &cell)| (x, y, cell)))
    }
}

fn collect_part_nos(schematic: &Vec<Vec<char>>) -> Vec<usize> {
    let mut part_nos = vec![];

    let mut part_no = String::new();
    let mut touches_symbol = false;
    for (y, row) in schematic.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                ('0'..='9') => {
                    part_no.push(*cell);

                    neighbors(x, y, row.len(), schematic.len()).for_each(|(nx, ny)| {
                        let neighbor = schematic[ny][nx];
                        if neighbor != '.' && !neighbor.is_ascii_digit() {
                            touches_symbol = true;
                        }
                    });
                }
                _ => {
                    if !part_no.is_empty() && touches_symbol {
                        part_nos.push(part_no.parse().unwrap())
                    }
                    part_no.clear();
                    touches_symbol = false;
                }
            }
        }
        if !part_no.is_empty() && touches_symbol {
            part_nos.push(part_no.parse().unwrap())
        }
        part_no.clear();
        touches_symbol = false;
    }
    part_nos
}

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    // (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbors(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    itertools::unfold(NEIGHBORS.iter(), move |iter| {
        for (dx, dy) in iter.by_ref() {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                return Some((nx as usize, ny as usize));
            }
        }

        None
    })
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (4361, 467835));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (539590, 80703636));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

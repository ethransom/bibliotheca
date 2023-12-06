// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example03.txt");
const INPUT: &str = include_str!("input03.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let schematic = parse(input);

    let mut part_nos = vec![];

    let mut part_no = vec![];
    let mut touches_symbol = false;
    for (y, row) in schematic.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                ('0'..='9') => {
                    part_no.push(cell);

                    neighbors(x, y, row.len(), schematic.len()).for_each(|(nx, ny)| {
                        let neighbor = schematic[ny][nx];
                        if neighbor != '.' && !neighbor.is_ascii_digit() {
                            touches_symbol = true;
                        }
                    });
                }
                _ => {
                    if !part_no.is_empty() && touches_symbol {
                        part_nos.push(part_no.drain(0..).collect::<String>().parse().unwrap())
                    }
                    part_no.clear();
                    touches_symbol = false;
                }
            }
        }
        if !part_no.is_empty() && touches_symbol {
            part_nos.push(part_no.drain(0..).collect::<String>().parse().unwrap())
        }
        part_no.clear();
        touches_symbol = false;
    }

    (part_nos.iter().sum(), 0)
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
    assert_eq!(solve(EXAMPLE), (4361, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (539590, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

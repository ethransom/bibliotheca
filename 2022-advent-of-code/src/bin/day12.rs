#![feature(test)]

use std::collections::VecDeque;

extern crate test;

const EXAMPLE: &str = include_str!("example12.txt");
const INPUT: &str = include_str!("input12.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (heightmap, start, end) = parse(input);

    // dbg!(
    //     heightmap
    //         .iter()
    //         .map(|row| row.iter().join(", "))
    //         .collect::<Vec<_>>(),
    //     start,
    //     end
    // );

    let best_path = find_path(&heightmap, start, end).expect("no path found");

    let best_path_any_a = heightmap
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_x, &h)| h == 0)
                .flat_map(|(x, _h)| find_path(&heightmap, (x, y), end))
                .min()
        })
        .min()
        .expect("no path found");

    (best_path, best_path_any_a)
}

fn parse(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;
    let heightmap = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, b)| {
                    let b = match b {
                        b'S' => {
                            start = Some((x, y));
                            b'a'
                        }
                        b'E' => {
                            end = Some((x, y));
                            b'z'
                        }
                        b => b,
                    };
                    match b {
                        b'a'..=b'z' => b - b'a',
                        b => panic!("invalid byte: {}", b),
                    }
                })
                .collect()
        })
        .collect();

    (heightmap, start.unwrap(), end.unwrap())
}

fn find_path(
    heightmap: &Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut distances: Vec<Vec<Option<usize>>> =
        heightmap.iter().map(|row| vec![None; row.len()]).collect();

    distances[start.1][start.0] = Some(0);

    let mut visited: Vec<Vec<_>> = heightmap.iter().map(|row| vec![false; row.len()]).collect();

    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let dist = distances[node.1][node.0].unwrap();

        if node == end {
            break; // is this just an optimization? Or does it matter for correctness?
        }

        visited[node.1][node.0] = true;

        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let x = node.0 as i32 + dx;
            let y = node.1 as i32 + dy;

            if x < 0 || y < 0 || x >= heightmap[0].len() as i32 || y >= heightmap.len() as i32 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            // TODO: do we need this?????
            if visited[y][x] {
                // already visited
                continue;
            }

            if distances[y][x].is_some() {
                // already queued
                continue;
            }

            let (height, neighbor_height) = (heightmap[node.1][node.0], heightmap[y][x]);
            if neighbor_height > height + 1 {
                // unreachable, can only climb 1 unit at a time
                continue;
            }

            distances[y][x] = Some(dist + 1);
            queue.push_back((x, y));
        }
    }

    print_grid(&distances);

    distances[end.1][end.0]
}

fn print_grid(distances: &Vec<Vec<Option<usize>>>) {
    let width = distances
        .iter()
        .flat_map(|row| row.iter().flatten().max())
        .max()
        .unwrap()
        .to_string()
        .len();

    if distances.iter().map(|row| row.len()).max().unwrap() * (width + 1) > 100 {
        // portrait
        print!("{:width$} ", "", width = width + 1);
        for row in 0..distances.len() {
            print!("{:width$}", row, width = width + 1);
        }
        println!();

        print!("{:width$}╔", "", width = width + 1);
        for _ in 0..distances.len() {
            print!("{:═^width$}", "═", width = width + 1);
        }
        println!();

        for col in 0..distances[0].len() {
            print!("{:width$}║", col.to_string(), width = width + 1);

            for row in distances {
                let cell = row[col];
                print!(
                    "{:>width$}",
                    cell.map_or("-".to_owned(), |i| i.to_string()),
                    width = width + 1
                );
            }
            println!();
        }
    } else {
        // landscape
        print!("{:width$} ", "", width = width + 1);
        for row in 0..distances[0].len() {
            print!("{:width$}", row, width = width + 1);
        }
        println!();

        print!("{:pad_width$}╔", "", pad_width = width + 1);
        for _ in 0..distances[0].len() {
            print!("{:═^width$}", "═", width = width + 1);
        }
        println!();

        for (r, row) in distances.iter().enumerate() {
            print!("{:width$}║", r.to_string(), width = width + 1);

            for cell in row {
                print!(
                    "{:>width$}",
                    cell.map_or("-".to_owned(), |i| i.to_string()),
                    width = width + 1
                );
            }
            println!();
        }
    }
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (31, 29));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (472, 0));
    });
}

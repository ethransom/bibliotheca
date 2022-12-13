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

    let best_path = find_path(&heightmap, start, end);

    (best_path, 0)
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

fn find_path(heightmap: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut distances: Vec<Vec<Option<usize>>> =
        heightmap.iter().map(|row| vec![None; row.len()]).collect();

    distances[start.1][start.0] = Some(0);

    let mut visited: Vec<Vec<_>> = heightmap.iter().map(|row| vec![false; row.len()]).collect();

    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let dist = distances[node.1][node.0].unwrap();

        // println!("VISIT: {:?} distance {}", node, dist);

        if node == end {
            // println!("FOUND END: {:?} distance {}", node, dist);
            // return dist;
            // return Some(dist);
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

            // println!("examining neighbor {:?}", (x, y));

            // TODO: do we need this?????
            if visited[y][x] {
                // println!("already visited");
                // already visited
                continue;
            }

            if distances[y][x].is_some() {
                // println!("already queued");
                // already queued
                continue;
            }

            if heightmap[y][x]
                .checked_sub(heightmap[node.1][node.0])
                .map_or(true, |diff| diff > 1)
            {
                // println!(
                //     "was unreachable {} vs {}",
                //     heightmap[y][x], heightmap[node.1][node.0]
                // );
                // unreachable, can only climb 1 unit at a time
                continue;
            }

            // println!("distance to {:?} is {}", (x, y), dist + 1,);
            distances[y][x] = Some(dist + 1);
            queue.push_back((x, y));
        }
    }

    print_grid(&distances);

    distances[end.1][end.0].expect("no path found")
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
    assert_eq!(solve(EXAMPLE), (31, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}

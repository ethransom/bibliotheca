#![feature(test)]

use itertools::Itertools;

extern crate test;

const EXAMPLE: &str = include_str!("example12.txt");
const INPUT: &str = include_str!("input12.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (heightmap, start, end) = parse(input);

    dbg!(
        heightmap
            .iter()
            .map(|row| row.iter().join(", "))
            .collect::<Vec<_>>(),
        start,
        end
    );

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

    let mut queue = vec![(start, 0)];

    while let Some((i, _)) = queue.iter().enumerate().min_by_key(|(_i, (_n, d))| d) {
        let (node, dist) = queue.remove(i);

        println!("VISIT: {:?} reachable with cost ({})", node, dist);

        visited[node.1][node.0] = true;

        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let x = node.0 as i32 + dx;
            let y = node.1 as i32 + dy;

            if x < 0 || y < 0 || x >= heightmap[0].len() as i32 || y >= heightmap.len() as i32 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            println!("examining neighbor {:?}", (x, y));

            if visited[y][x] {
                println!("already visited");
                // already visited
                continue;
            }

            if u8::abs_diff(heightmap[y][x], heightmap[node.1][node.0]) > 1 {
                println!(
                    "was unreachable {} vs {}",
                    heightmap[y][x], heightmap[node.1][node.0]
                );
                // unreachable, can only climb 1 unit at a time
                continue;
            }

            if distances[y][x].is_none() || distances[y][x].unwrap() > dist + 1 {
                println!(
                    "better path to {:?} found cost {} vs {:?}",
                    (x, y),
                    dist + 1,
                    distances[y][x]
                );
                distances[y][x] = Some(dist + 1);
                queue.push(((x, y), dist + 1));
            }
        }
    }

    dbg!(distances
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| cell.map_or("-".to_owned(), |i| i.to_string()))
                .join(", ")
        })
        .collect::<Vec<_>>());

    distances[end.1][end.0].expect("no path found")
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

// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, HashSet, VecDeque};

const EXAMPLE: &str = include_str!("example10.txt");
const INPUT: &str = include_str!("input10.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (i64, i64) {
    let map = Map::parse(input);

    println!("{}", map.serialize());

    // BFS, as it guarantees the shortest path
    let mut queue = VecDeque::new();
    queue.push_back(map.start);
    let mut visited = HashMap::new();
    visited.insert(map.start, 0);

    while let Some(pos) = queue.pop_front() {
        let dist = visited[&pos];
        for neighbor in map.neighbors(pos) {
            if visited.contains_key(&neighbor) {
                continue;
            }
            visited.insert(neighbor, dist + 1);
            queue.push_back(neighbor);
        }
    }

    println!("nodes reachable: {}", visited.len());

    let main_loop_map = {
        let mut main_loop_map = map.clone();
        main_loop_map
            .edges
            .retain(|e| visited.contains_key(&e.0) && visited.contains_key(&e.1));
        main_loop_map
    };

    let [all_nodes, loop_nodes] = [&map, &main_loop_map].map(|m| m.serialize_fancy());

    all_nodes
        .lines()
        .zip(loop_nodes.lines())
        .for_each(|(left, right)| {
            println!("{}\t\t{}", left, right);
        });

    let &max = visited.values().max().expect("no pipes reachable");

    // PART 2
    let main_loop: HashSet<(i64, i64)> = visited.into_keys().collect();

    println!("BOUNDS: {:?}", main_loop_map.bounds);

    // BFS part two, although DFS would probably also work fine
    let mut queue = VecDeque::new();
    queue.push_back(map.start);
    let mut visited = main_loop.clone();
    let mut interiors = vec![];

    for pos in &main_loop {
        let neighbors = [
            (pos.0, pos.1 - 1), // never
            (pos.0 + 1, pos.1), // eat
            (pos.0, pos.1 + 1), // soggy
            (pos.0 - 1, pos.1), // waffles
        ];
        for neighbor in neighbors {
            if !map.in_bounds(neighbor) {
                continue;
            }
            if visited.contains(&neighbor) {
                continue;
            }

            // 'flood'
            let mut outside = false;
            let mut seen = HashSet::new();
            seen.insert(neighbor);
            let mut queue = VecDeque::new();
            queue.push_back(neighbor);

            while let Some(pos) = queue.pop_front() {
                let neighbors = [
                    (pos.0, pos.1 - 1), // never
                    (pos.0 + 1, pos.1), // eat
                    (pos.0, pos.1 + 1), // soggy
                    (pos.0 - 1, pos.1), // waffles
                ];
                for neighbor in neighbors {
                    if !map.in_bounds(neighbor) {
                        outside = true;
                        continue;
                    }
                    if visited.contains(&neighbor) {
                        continue;
                    }
                    if !seen.insert(neighbor) {
                        continue;
                    }
                    queue.push_back(neighbor);
                }
            }

            visited.extend(&seen);
            if !outside {
                interiors.push(seen);
            }
        }
    }

    let inside = interiors.iter().map(|i| i.len()).sum::<usize>();
    println!("nodes on inside: {:?}", inside);

    println!(
        "{}",
        main_loop_map.serialize_inside_fancy(
            &visited,
            &main_loop,
            &interiors
                .into_iter()
                .reduce(|mut a, b| {
                    a.extend(b);
                    a
                })
                .unwrap()
        )
    );

    (max, inside as i64)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    edges: HashSet<((i64, i64), (i64, i64))>,
    start: (i64, i64),
    bounds: ((i64, i64), (i64, i64)),
}

impl Map {
    fn in_bounds(&self, (x, y): (i64, i64)) -> bool {
        let ((x_min, x_max), (y_min, y_max)) = self.bounds;

        // dbg!((x, y));

        x >= x_min && x <= x_max && y >= y_min && y <= y_max
    }

    fn neighbors(&self, pos: (i64, i64)) -> Vec<(i64, i64)> {
        [
            (pos.0, pos.1 - 1), // never
            (pos.0 + 1, pos.1), // eat
            (pos.0, pos.1 + 1), // soggy
            (pos.0 - 1, pos.1), // waffles
        ]
        .into_iter()
        .filter(|dst| self.has_edge(pos, *dst))
        .collect()
    }

    fn has_edge(&self, src: (i64, i64), dst: (i64, i64)) -> bool {
        self.edges.contains(&(src, dst))
    }

    fn directions(&self, pos: (i64, i64)) -> [bool; 4] {
        [
            (pos.0, pos.1 - 1), // never
            (pos.0 + 1, pos.1), // eat
            (pos.0, pos.1 + 1), // soggy
            (pos.0 - 1, pos.1), // waffles
        ]
        .map(|dst| self.edges.contains(&(pos, dst)))
    }

    fn serialize(&self) -> String {
        let mut buf = String::new();

        let ((x_min, x_max), (y_min, y_max)) = self.bounds;
        for y in y_min..=y_max {
            if y != 0 {
                buf.push('\n');
            }
            for x in x_min..=x_max {
                let pos = (x, y);
                if self.start == pos {
                    buf.push('S');
                } else {
                    match self.directions(pos) {
                        // | is a vertical pipe connecting north and south.
                        [true, false, true, false] => buf.push('|'),
                        // - is a horizontal pipe connecting east and west.
                        [false, true, false, true] => buf.push('-'),
                        // L is a 90-degree bend connecting north and east.
                        [true, true, false, false] => buf.push('L'),
                        // J is a 90-degree bend connecting north and west.
                        [true, false, false, true] => buf.push('J'),
                        // 7 is a 90-degree bend connecting south and west.
                        [false, false, true, true] => buf.push('7'),
                        // F is a 90-degree bend connecting south and east.
                        [false, true, true, false] => buf.push('F'),
                        // . is ground; there is no pipe in this tile.
                        [false, false, false, false] => buf.push('.'),

                        _ => panic!("unknown neighbors: {:?}", self.directions(pos)),
                    }
                }
            }
        }

        buf
    }

    fn serialize_fancy(&self) -> String {
        let mut buf = String::new();

        let ((x_min, x_max), (y_min, y_max)) = self.bounds;
        for y in y_min..=y_max {
            if y != 0 {
                buf.push('\n');
            }
            for x in x_min..=x_max {
                let pos = (x, y);
                let c = if self.start == pos {
                    'S'
                } else {
                    match self.directions(pos) {
                        // | is a vertical pipe connecting north and south.
                        [true, false, true, false] => '┃',
                        // - is a horizontal pipe connecting east and west.
                        [false, true, false, true] => '━',
                        // L is a 90-degree bend connecting north and east.
                        [true, true, false, false] => '┗',
                        // J is a 90-degree bend connecting north and west.
                        [true, false, false, true] => '┛',
                        // 7 is a 90-degree bend connecting south and west.
                        [false, false, true, true] => '┓',
                        // F is a 90-degree bend connecting south and east.
                        [false, true, true, false] => '┏',
                        // . is ground; there is no pipe in this tile.
                        [false, false, false, false] => ' ',

                        _ => panic!("unknown neighbors: {:?}", self.directions(pos)),
                    }
                };
                buf.push(c);
            }
        }

        buf
    }

    fn serialize_inside_fancy(
        &self,
        outside: &HashSet<(i64, i64)>,
        main_loop: &HashSet<(i64, i64)>,
        interiors: &HashSet<(i64, i64)>,
    ) -> String {
        let mut buf = String::new();

        let ((x_min, x_max), (y_min, y_max)) = self.bounds;
        for y in y_min..=y_max {
            if y != 0 {
                buf.push('\n');
            }
            for x in x_min..=x_max {
                let pos = (x, y);
                let c = if main_loop.contains(&pos) {
                    '█'
                } else if interiors.contains(&pos) {
                    '*'
                } else if outside.contains(&pos) {
                    '░'
                } else if self.start == pos {
                    'S'
                } else {
                    match self.directions(pos) {
                        // | is a vertical pipe connecting north and south.
                        [true, false, true, false] => '┃',
                        // - is a horizontal pipe connecting east and west.
                        [false, true, false, true] => '━',
                        // L is a 90-degree bend connecting north and east.
                        [true, true, false, false] => '┗',
                        // J is a 90-degree bend connecting north and west.
                        [true, false, false, true] => '┛',
                        // 7 is a 90-degree bend connecting south and west.
                        [false, false, true, true] => '┓',
                        // F is a 90-degree bend connecting south and east.
                        [false, true, true, false] => '┏',
                        // . is ground; there is no pipe in this tile.
                        [false, false, false, false] => ' ',

                        _ => panic!("unknown neighbors: {:?}", self.directions(pos)),
                    }
                };
                buf.push(c);
            }
        }

        buf
    }

    fn parse(input: &str) -> Map {
        let mut start = None;
        let mut edges = HashSet::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let pos = (x as i64, y as i64);
                match c {
                    // | is a vertical pipe connecting north and south.
                    '|' => {
                        edges.insert((pos, (pos.0, pos.1 + 1)));
                        edges.insert((pos, (pos.0, pos.1 - 1)));
                    }
                    // - is a horizontal pipe connecting east and west.
                    '-' => {
                        edges.insert((pos, (pos.0 + 1, pos.1)));
                        edges.insert((pos, (pos.0 - 1, pos.1)));
                    }
                    // L is a 90-degree bend connecting north and east.
                    'L' => {
                        edges.insert((pos, (pos.0, pos.1 - 1)));
                        edges.insert((pos, (pos.0 + 1, pos.1)));
                    }
                    // J is a 90-degree bend connecting north and west.
                    'J' => {
                        edges.insert((pos, (pos.0, pos.1 - 1)));
                        edges.insert((pos, (pos.0 - 1, pos.1)));
                    }
                    // 7 is a 90-degree bend connecting south and west.
                    '7' => {
                        edges.insert((pos, (pos.0, pos.1 + 1)));
                        edges.insert((pos, (pos.0 - 1, pos.1)));
                    }
                    // F is a 90-degree bend connecting south and east.
                    'F' => {
                        edges.insert((pos, (pos.0, pos.1 + 1)));
                        edges.insert((pos, (pos.0 + 1, pos.1)));
                    }
                    // . is ground; there is no pipe in this tile.
                    '.' => {}
                    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
                    'S' => {
                        start = Some(pos);
                    }
                    _ => panic!("unknown char: {}", c),
                }
            })
        });

        let start = start.expect("no start found");

        // create edges leading out of start
        for neighbors in [
            (start.0, start.1 - 1), // never
            (start.0 + 1, start.1), // eat
            (start.0, start.1 + 1), // soggy
            (start.0 - 1, start.1), // waffles
        ] {
            if !edges.contains(&(neighbors, start)) {
                continue;
            }
            edges.insert((start, neighbors));
        }

        let bounds = edges.iter().fold(
            ((0, 0), (0, 0)),
            |((x_min, x_max), (y_min, y_max)), ((x, y), _)| {
                (
                    (x_min.min(*x), x_max.max(*x)),
                    (y_min.min(*y), y_max.max(*y)),
                )
            },
        );

        Map {
            edges,
            start,
            bounds,
        }
    }
}

#[test]
fn test_parse_unparse() {
    let map = Map::parse(EXAMPLE);
    assert_eq!(map.serialize(), EXAMPLE);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (8, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (7102, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

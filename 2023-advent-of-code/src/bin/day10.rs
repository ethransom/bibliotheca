// #![feature(test)]

// extern crate test;

use std::collections::HashSet;

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
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(map.start);
    let mut visited = std::collections::HashMap::new();
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

    let ((x_min, x_max), (y_min, y_max)) = map.edges.iter().fold(
        ((0, 0), (0, 0)),
        |((x_min, x_max), (y_min, y_max)), ((x, y), _)| {
            (
                (x_min.min(*x), x_max.max(*x)),
                (y_min.min(*y), y_max.max(*y)),
            )
        },
    );

    println!("nodes reachable: {}", visited.len());

    let (all_nodes, loop_nodes) = (
        map.serialize_fancy(),
        map.serialize_fancy_only_main_loop(&visited),
    );

    all_nodes
        .lines()
        .zip(loop_nodes.lines())
        .for_each(|(left, right)| {
            println!("{}\t\t{}", left, right);
        });

    let mut buf = String::new();

    for y in y_min..=y_max {
        if y != 0 {
            buf.push('\n');
        }
        for x in x_min..=x_max {
            let pos = (x, y);
            if map.start == pos {
                buf.push('S');
            } else if let Some(dist) = visited.get(&pos) {
                buf.push_str(&format!("{}", dist));
            } else {
                buf.push(' ');
            }
        }
    }

    let &max = visited.values().max().expect("no pipes reachable");

    (max, 0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    edges: HashSet<((i64, i64), (i64, i64))>,
    start: (i64, i64),
}

impl Map {
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
        let ((x_min, x_max), (y_min, y_max)) = self.edges.iter().fold(
            ((0, 0), (0, 0)),
            |((x_min, x_max), (y_min, y_max)), ((x, y), _)| {
                (
                    (x_min.min(*x), x_max.max(*x)),
                    (y_min.min(*y), y_max.max(*y)),
                )
            },
        );

        let mut buf = String::new();

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
        let ((x_min, x_max), (y_min, y_max)) = self.edges.iter().fold(
            ((0, 0), (0, 0)),
            |((x_min, x_max), (y_min, y_max)), ((x, y), _)| {
                (
                    (x_min.min(*x), x_max.max(*x)),
                    (y_min.min(*y), y_max.max(*y)),
                )
            },
        );

        let mut buf = String::new();

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
                        [true, false, true, false] => buf.push('┃'),
                        // - is a horizontal pipe connecting east and west.
                        [false, true, false, true] => buf.push('━'),
                        // L is a 90-degree bend connecting north and east.
                        [true, true, false, false] => buf.push('┗'),
                        // J is a 90-degree bend connecting north and west.
                        [true, false, false, true] => buf.push('┛'),
                        // 7 is a 90-degree bend connecting south and west.
                        [false, false, true, true] => buf.push('┓'),
                        // F is a 90-degree bend connecting south and east.
                        [false, true, true, false] => buf.push('┏'),
                        // . is ground; there is no pipe in this tile.
                        [false, false, false, false] => buf.push(' '),

                        _ => panic!("unknown neighbors: {:?}", self.directions(pos)),
                    }
                }
            }
        }

        buf
    }

    fn serialize_fancy_only_main_loop(
        &self,
        main_loop: &std::collections::HashMap<(i64, i64), i64>,
    ) -> String {
        let ((x_min, x_max), (y_min, y_max)) = self.edges.iter().fold(
            ((0, 0), (0, 0)),
            |((x_min, x_max), (y_min, y_max)), ((x, y), _)| {
                (
                    (x_min.min(*x), x_max.max(*x)),
                    (y_min.min(*y), y_max.max(*y)),
                )
            },
        );

        let mut buf = String::new();

        for y in y_min..=y_max {
            if y != 0 {
                buf.push('\n');
            }
            for x in x_min..=x_max {
                let pos = (x, y);
                if self.start == pos {
                    buf.push('S');
                } else if !main_loop.contains_key(&pos) {
                    buf.push(' ');
                } else {
                    match self.directions(pos) {
                        // | is a vertical pipe connecting north and south.
                        [true, false, true, false] => buf.push('┃'),
                        // - is a horizontal pipe connecting east and west.
                        [false, true, false, true] => buf.push('━'),
                        // L is a 90-degree bend connecting north and east.
                        [true, true, false, false] => buf.push('┗'),
                        // J is a 90-degree bend connecting north and west.
                        [true, false, false, true] => buf.push('┛'),
                        // 7 is a 90-degree bend connecting south and west.
                        [false, false, true, true] => buf.push('┓'),
                        // F is a 90-degree bend connecting south and east.
                        [false, true, true, false] => buf.push('┏'),
                        // . is ground; there is no pipe in this tile.
                        [false, false, false, false] => buf.push(' '),

                        _ => panic!("unknown neighbors: {:?}", self.directions(pos)),
                    }
                }
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

        Map { edges, start }
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

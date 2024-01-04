#![feature(inline_const)]
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
    let map = main_loop_map;
    let mut inside = HashSet::new();

    let ((x_min, x_max), (y_min, y_max)) = map.bounds;

    for y in y_min..=y_max {
        let mut in_region = false;
        let [mut line, mut r, mut i] = [const { String::new() }; 3];
        for x in x_min..=x_max {
            let pos = (x, y);
            match map.directions(pos) {
                // |
                [true, false, true, false] => in_region = !in_region,
                // -
                [false, true, false, true] => {}
                // L
                [true, true, false, false] => {}
                // J
                [true, false, false, true] => {}
                // 7
                [false, false, true, true] => in_region = !in_region,
                // F
                [false, true, true, false] => in_region = !in_region,
                // .
                [false, false, false, false] => {
                    if in_region {
                        inside.insert(pos);
                    }
                }

                other => panic!("unknown neighbors: {:?}", other),
            };
            let c = if map.start == pos {
                'S'
            } else {
                match map.directions(pos) {
                    // | is a vertical pipe connecting north and south.
                    [true, false, true, false] => '|',
                    // - is a horizontal pipe connecting east and west.
                    [false, true, false, true] => '-',
                    // L is a 90-degree bend connecting north and east.
                    [true, true, false, false] => 'L',
                    // J is a 90-degree bend connecting north and west.
                    [true, false, false, true] => 'J',
                    // 7 is a 90-degree bend connecting south and west.
                    [false, false, true, true] => '7',
                    // F is a 90-degree bend connecting south and east.
                    [false, true, true, false] => 'F',
                    // . is ground; there is no pipe in this tile.
                    [false, false, false, false] => '.',

                    _ => panic!("unknown neighbors: {:?}", map.directions(pos)),
                }
            };
            line.push(c);
            r.push(if in_region { 'I' } else { 'O' });
            i.push(if inside.contains(&pos) { '*' } else { ' ' });
        }
        println!("{line}\n{r}\n{i}\n"); // NOTE: double newline
    }

    println!("{}", map.serialize_inside_fancy(&inside));
    println!("enclosed area: {inside}", inside = inside.len());

    (max, inside.len() as i64)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    edges: HashSet<((i64, i64), (i64, i64))>,
    start: (i64, i64),
    bounds: ((i64, i64), (i64, i64)),
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
                        [true, false, true, false] => '|',
                        // - is a horizontal pipe connecting east and west.
                        [false, true, false, true] => '-',
                        // L is a 90-degree bend connecting north and east.
                        [true, true, false, false] => 'L',
                        // J is a 90-degree bend connecting north and west.
                        [true, false, false, true] => 'J',
                        // 7 is a 90-degree bend connecting south and west.
                        [false, false, true, true] => '7',
                        // F is a 90-degree bend connecting south and east.
                        [false, true, true, false] => 'F',
                        // . is ground; there is no pipe in this tile.
                        [false, false, false, false] => '.',

                        other => panic!("unknown neighbors: {:?}", other),
                    }
                };
                buf.push(c);
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

                        other => panic!("unknown neighbors: {:?}", other),
                    }
                };
                buf.push(c);
            }
        }

        buf
    }

    fn serialize_inside_fancy(&self, interiors: &HashSet<(i64, i64)>) -> String {
        let mut buf = String::new();

        let ((x_min, x_max), (y_min, y_max)) = self.bounds;
        for y in y_min..=y_max {
            if y != 0 {
                buf.push('\n');
            }
            for x in x_min..=x_max {
                let pos = (x, y);
                let c = if interiors.contains(&pos) {
                    'I'
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

                        other => panic!("unknown neighbors: {:?}", other),
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
    assert_eq!(solve(EXAMPLE), (8, 1));
}

#[test]
fn test_part2_examples() {
    assert_eq!(
        solve(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
        )
        .1,
        4
    );

    assert_eq!(
        solve(
            "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
        )
        .1,
        4
    );

    assert_eq!(
        solve(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
        )
        .1,
        8
    );

    assert_eq!(
        solve(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
        )
        .1,
        10
    );
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (7102, 363));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

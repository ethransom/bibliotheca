#![feature(let_chains)]
#![feature(iter_map_windows)]
// #![feature(test)]

// extern crate test;

use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use std::collections::VecDeque;

const EXAMPLE: &str = include_str!("example17.txt");
const INPUT: &str = include_str!("input17.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const STRAIGHT_LINE_MAX: usize = 3;

const DIRECTIONS: &[Dir] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn solve(input: &str) -> (usize, usize) {
    let map = Map::parse(input);

    println!("{}", map);

    let start = (0, 0);
    let end = (map.width - 1, map.height - 1);

    let crucible_loss = min_heat_loss(&map, start, end, Map::crucible_neighbors, 0);

    let ultra_crucible_loss = min_heat_loss(&map, start, end, Map::ultra_crucible_neighbors, 4);

    (crucible_loss, ultra_crucible_loss)
}

type NeighborsFn = fn(&Map, &Point, Dir, usize) -> Vec<(Point, Dir, usize)>;

fn min_heat_loss(
    map: &Map,
    start: (usize, usize),
    end: (usize, usize),
    neighbors: NeighborsFn,
    min: usize,
) -> usize {
    let mut distances = HashMap::<(Point, Dir, usize), usize>::default();
    for &dir in DIRECTIONS {
        for step in 0..=10 {
            distances.insert((start, dir, step), 0);
        }
    }
    let mut previous = HashMap::<(Point, Dir, usize), (Point, Dir, usize)>::default();
    let mut unvisited = vec![];
    unvisited.push((start, (0, 1), 0usize, 0));
    unvisited.push((start, (1, 0), 0usize, 0));
    while let Some((current, current_dir, current_steps, _dist)) = unvisited
        .iter()
        .position_min_by(|&(_, _, _, a), &(_, _, _, b)| a.cmp(b))
        .map(|pos| unvisited.swap_remove(pos))
    {
        let dist = distances.get(&(current, current_dir, current_steps));
        // println!("visiting {current:?} {current_dir:?} {current_steps} @ distance {dist:?}");

        let &dist = dist.expect("uhhhh, 'unrechable' much??");

        for (neighbor, neighbor_dir, neighbor_steps) in
            neighbors(map, &current, current_dir, current_steps)
        {
            let alt = dist + map.loss[&neighbor] as usize;
            let last = distances.get(&(neighbor, neighbor_dir, neighbor_steps));
            // println!("\tneighbor of {neighbor:?} {neighbor_dir:?} {neighbor_steps}, previously reachable with {last:?} now reachable with {alt}");
            if last.map_or(true, |&distance| alt < distance) {
                distances.insert((neighbor, neighbor_dir, neighbor_steps), alt);
                previous.insert(
                    (neighbor, neighbor_dir, neighbor_steps),
                    (current, current_dir, current_steps),
                );
                unvisited.push((neighbor, neighbor_dir, neighbor_steps, alt));
            }
        }
    }

    println!("{len} total reachable", len = distances.len());

    let foo = distances
        .iter()
        .filter(|&(&(pos, _dir, _steps), _dist)| pos == end)
        .collect_vec();
    println!("solutions: {foo:?}");

    let (&(final_point, final_dir, final_steps), &final_dist) = distances
        .iter()
        .filter(|&(&(pos, _dir, steps), _dist)| pos == end && steps >= min)
        .min_by(|&((_, _, _), a_dist), &((_, _, _), b_dist)| a_dist.cmp(b_dist))
        .expect("no solution");
    println!("SOLVEDDDD with a distance of {final_dist:?}");

    let mut path = VecDeque::new(); // purely so we don't have to reverse
    let mut point = (final_point, final_dir, final_steps);
    while point.0 != start {
        path.push_front(point);
        point = previous[&point];
    }

    println!("PATH:");
    for p in &path {
        println!("{p:?} cost: {cost}", cost = map.loss[&p.0]);
    }

    println!();

    for y in 0..map.height {
        for x in 0..map.width {
            let c = if let Some((_point, dir, _steps)) =
                path.iter().find(|(point, _, _)| point == &(x, y))
            {
                match dir {
                    (0, 1) => 'v',
                    (-1, 0) => '<',
                    (1, 0) => '>',
                    (0, -1) => '^',
                    _ => panic!(),
                }
                .to_string()
            } else {
                map.loss[&(x, y)].to_string()
            };
            print!("{}", c);
        }
        println!();
    }
    final_dist
}

type Point = (usize, usize);

type Dir = (i64, i64);

struct Map {
    loss: HashMap<Point, u8>,
    height: usize,
    width: usize,
}

impl Map {
    fn parse(input: &str) -> Map {
        let loss = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(move |(x, c)| Some(((x, y), c.to_digit(10).unwrap() as u8)))
            })
            .collect::<HashMap<_, _>>();

        let (width, height) = loss.keys().fold((0, 0), |(max_x, max_y), &(x, y)| {
            (max_x.max(x), max_y.max(y))
        });

        let (width, height) = (width + 1, height + 1);

        Map {
            loss,
            width,
            height,
        }
    }

    fn crucible_neighbors(
        &self,
        point: &Point,
        point_dir: Dir,
        point_steps: usize,
    ) -> Vec<(Point, Dir, usize)> {
        let &(x, y) = point;

        if !self.loss.contains_key(&(x, y)) {
            panic!("tried neighbors of off-grid");
        }

        DIRECTIONS
            .iter()
            .cloned()
            .filter_map(|(dx, dy)| {
                let steps = if point_dir == (dx, dy) {
                    // going straight
                    let steps = point_steps + 1;
                    if steps > STRAIGHT_LINE_MAX {
                        return None;
                    }
                    steps
                } else if point_dir == (-dx, -dy) {
                    // no backtrack
                    return None;
                } else {
                    1
                };

                if let Ok(x) = (x as i64 + dx).try_into()
                    && let Ok(y) = (y as i64 + dy).try_into()
                    && self.loss.contains_key(&(x, y))
                {
                    return Some(((x, y), (dx, dy), steps));
                }

                None
            })
            .collect()
    }

    fn ultra_crucible_neighbors(
        &self,
        point: &Point,
        point_dir: Dir,
        point_steps: usize,
    ) -> Vec<(Point, Dir, usize)> {
        let &(x, y) = point;

        if !self.loss.contains_key(&(x, y)) {
            panic!("tried neighbors of off-grid");
        }

        DIRECTIONS
            .iter()
            .cloned()
            .filter_map(|(dx, dy)| {
                let steps = if point_steps >= 4 {
                    if point_dir == (dx, dy) {
                        // going straight
                        let steps = point_steps + 1;
                        if steps > 10 {
                            return None;
                        }
                        steps
                    } else if point_dir == (-dx, -dy) {
                        // no backtrack
                        return None;
                    } else {
                        1
                    }
                } else {
                    // must go straight
                    if point_dir != (dx, dy) {
                        return None;
                    }
                    point_steps + 1
                };

                if let Ok(x) = (x as i64 + dx).try_into()
                    && let Ok(y) = (y as i64 + dy).try_into()
                    && self.loss.contains_key(&(x, y))
                {
                    return Some(((x, y), (dx, dy), steps));
                }

                None
            })
            .collect()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                writeln!(f)?;
            }
            for x in 0..self.width {
                write!(f, "{}", self.loss[&(x, y)])?;
            }
        }

        Ok(())
    }
}

#[test]
fn test_parse_debug() {
    assert_eq!(format!("{}", Map::parse(EXAMPLE)), EXAMPLE);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (102, 94));
}

#[test]
fn test_subreddit_example() {
    let map = Map::parse("112999\n911111");

    println!("{}", map);

    let start = (0, 0);
    let end = (map.width - 1, map.height - 1);

    let crucible_loss = min_heat_loss(&map, start, end, Map::crucible_neighbors, 0);

    assert_eq!(crucible_loss, 7);
}

#[test]
fn test_ultra_crucible_example() {
    let map = Map::parse(
        "111111111111
999999999991
999999999991
999999999991
999999999991",
    );
    assert_eq!(
        min_heat_loss(
            &map,
            (0, 0),
            (map.width - 1, map.height - 1),
            Map::ultra_crucible_neighbors,
            4
        ),
        71
    );
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (956, 1106));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

// #![feature(test)]

// extern crate test;

use fxhash::FxHashSet as HashSet;
use itertools::Itertools;

const EXAMPLE: &str = include_str!("example25.txt");
const INPUT: &str = include_str!("input25.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let graph = parse(input);

    println!(
        "graph of {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );

    let bisection = find_bisection(&graph).expect("couldn't find bisection");

    (bisection[0] * bisection[1], 0)
}

fn find_bisection(graph: &Graph) -> Option<[usize; 2]> {
    let combinations = graph.edges.iter().tuple_combinations();
    let num_combinations = combinations.size_hint();
    println!("trying {num_combinations:?} combinations");
    let mut i = 0;
    for (&a, &b, &c) in combinations {
        i += 1;
        if i % 1_000_000 == 0 {
            println!("{i}");
        }
        assert!(a != b && b != c);
        let mut graph = graph.clone();
        graph.remove_edge(&a);
        graph.remove_edge(&b);
        graph.remove_edge(&c);

        // check connectedness
        let mut components: Vec<HashSet<_>> = vec![];
        let mut unvisited = graph.nodes.clone();
        while !unvisited.is_empty() {
            let mut visited = HashSet::default();
            let mut stack = vec![];
            stack.push(unvisited.iter().cloned().next().unwrap());
            while let Some(node) = stack.pop() {
                // println!("\tvisiting {node}");
                visited.insert(node);
                unvisited.remove(node);
                for neighbor in graph.neighbors(node) {
                    // println!("\t\tneighbor of {neighbor}");
                    if !visited.contains(neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
            components.push(visited);
        }

        let components = components
            .iter()
            .map(|component| component.len())
            .collect_vec();

        // println!(
        //     "deleting {a:?}, {b:?}, {c:?} resulted in components of {:?} out of {} elements",
        //     components,
        //     graph.nodes.len(),
        // );

        if components.len() == 2 {
            return Some(components.try_into().unwrap());
        }
    }

    None
}

#[derive(Debug, Clone, Default)]
struct Graph<'a> {
    nodes: HashSet<&'a str>, // redundant?
    edges: HashSet<(&'a str, &'a str)>,
}

impl<'a> Graph<'a> {
    fn remove_edge(&mut self, &(from, to): &(&'a str, &'a str)) {
        // might not need to check inverse?
        self.edges.retain(|&e| e != (from, to) && e != (to, from));
    }

    fn neighbors(&'a self, node: &'a str) -> impl Iterator<Item = &&str> {
        self.edges.iter().filter_map(move |(from, to)| {
            if *from == node {
                Some(to)
            } else if *to == node {
                Some(from)
            } else {
                None
            }
        })
    }
}

fn parse(input: &str) -> Graph {
    println!("PARSE");
    let mut graph = Graph::default();
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();

        graph.nodes.insert(left);
        println!("{left} -> {right}:");

        for r in right.split(' ') {
            println!("\t{r} ({left}, {r})");
            graph.nodes.insert(r);
            graph.edges.insert((left, r));
        }
    }

    graph
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (54, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (0, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

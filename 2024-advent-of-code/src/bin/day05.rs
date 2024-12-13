// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, HashSet, VecDeque};

const EXAMPLE: &str = include_str!("example05.txt");
const INPUT: &str = include_str!("input05.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    // dbg!(solve(INPUT));
}

fn is_update_valid(update: &[&str], rules: &HashMap<&str, Vec<&str>>) -> bool {
    for i in 1..=update.len() {
        let set = &update[..i];
        let preceding = &set[..set.len() - 1];
        let target = set[set.len() - 1];

        if preceding.is_empty() {
            continue;
        }

        for p in preceding {
            // here, we have `target` coming after `p`
            // is there a rule that has `target` -> `p`?

            if rules.get(target).is_some_and(|v| v.contains(p)) {
                return false;
            }
        }
    }

    true
}

fn solve(input: &str) -> (usize, usize) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules: Vec<(&str, &str)> = rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .collect();

    let mut rule_map = HashMap::new();
    for &(left, right) in &rules {
        rule_map
            .entry(left)
            .and_modify(|deps: &mut Vec<&str>| deps.push(right))
            .or_insert_with(|| vec![right]);
    }

    let updates: Vec<Vec<&str>> = updates
        .lines()
        .map(|line| line.split(",").collect())
        .collect();

    let all_nodes = HashSet::<&str>::from_iter(
        rules
            .iter()
            .flat_map(|&(left, right)| [left, right].into_iter()),
    );

    // println!("{all_nodes:?}");

    let mut rules2 = rules.clone();
    let mut sorted = vec![];
    let mut s = all_nodes.clone();
    for (_from, to) in &rules {
        s.remove(to);
    }
    // let mut s = VecDeque::from_iter(s.into_iter());
    println!("starting edges: {s:?}");
    while let Some(&n) = s.iter().next() {
        s.remove(n);

        sorted.push(n);

        println!("considering source node {n}");

        // dbg!(&sorted, &s, &rules2);

        // for each node m with an edge e from n to m do
        while let Some(i) = rules2.iter().position(|&(f, _t)| f == n) {
            // remove edge e from the graph
            let (_n, m) = rules2.remove(i);
            println!("edge from {n} to {m}");

            println!("\t{rules2:?}");

            // if m has no other incoming edges then
            if rules2.iter().all(|&(_f, t)| t != m) {
                println!("\t{m} is now source, queueing...");
                // insert m into S
                s.insert(m);
            }
        }
    }
    println!("remainder: {rules2:?}");
    println!("sorted: {sorted:?}");

    if !rules2.is_empty() {
        panic!("rules had cycle");
    }

    let sorted_map: HashMap<_, _> = sorted
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    let (good_updates, bad_updates) = updates
        .into_iter()
        .partition::<Vec<_>, _>(|update| is_update_valid(update, &rule_map));

    dbg!(&bad_updates);

    let reordered_updates: Vec<_> = bad_updates
        .into_iter()
        .map(|mut update| {
            update.sort_by_key(|v| sorted_map.get(dbg!(v)).unwrap());

            update
        })
        .collect();

    [good_updates, reordered_updates]
        .map(|updates| {
            updates
                .into_iter()
                .map(|update| update[update.len() / 2].parse::<usize>().unwrap())
                .sum()
        })
        .into()
}

fn topo_sort<'a>(rules: &[(&'a str, &'a str)]) -> Vec<&'a str> {
    let all_nodes = HashSet::<&str>::from_iter(
        rules
            .iter()
            .flat_map(|&(left, right)| [left, right].into_iter()),
    );

    let mut rules2 = Vec::from(rules);
    let mut sorted = vec![];
    let mut s = all_nodes.clone();
    for (_from, to) in rules {
        s.remove(to);
    }
    // let mut s = VecDeque::from_iter(s.into_iter());
    // println!("starting edges: {s:?}");
    while let Some(&n) = s.iter().next() {
        s.remove(n);

        sorted.push(n);

        // println!("considering source node {n}");

        // dbg!(&sorted, &s, &rules2);

        // for each node m with an edge e from n to m do
        while let Some(i) = rules2.iter().position(|&(f, _t)| f == n) {
            // remove edge e from the graph
            let (_n, m) = rules2.remove(i);
            // println!("edge from {n} to {m}");

            // if m has no other incoming edges then
            if rules2.iter().any(|&(_f, t)| t == m) {
                // println!("\t{m} is now source, queueing...");
                // insert m into S
                s.insert(m);
            }
        }
    }
    // println!("sorted: {sorted:?}");

    let mut seen = HashSet::new();
    let sorted = sorted
        .into_iter()
        .filter(|&s| seen.insert(s))
        .collect::<Vec<_>>();

    // println!("sorted deduped: {sorted:?}");

    sorted
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (143, 123));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (4924, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example05.txt");
const INPUT: &str = include_str!("input05.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
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

    let (good_updates, bad_updates) = updates
        .into_iter()
        .partition::<Vec<_>, _>(|update| is_update_valid(update, &rule_map));

    let reordered_updates: Vec<_> = bad_updates
        .into_iter()
        .map(|update| {
            topo_sort(
                &rules
                    .iter()
                    .cloned()
                    .filter(|(t, f)| update.contains(t) && update.contains(f))
                    .collect::<Vec<_>>(),
            )
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

    let mut rules = Vec::from(rules);
    let sinks = rules.iter().map(|&(_f, t)| t).collect::<HashSet<_>>();
    let mut s = all_nodes
        .difference(&sinks)
        .cloned()
        .collect::<HashSet<_>>();

    let mut sorted = vec![];

    while let Some(&n) = s.iter().next() {
        s.remove(n);

        sorted.push(n);

        // for each node m with an edge e from n to m do
        while let Some(i) = rules.iter().position(|&(f, _t)| f == n) {
            // remove edge e from the graph
            let (_n, m) = rules.remove(i);

            // if m has no other incoming edges then
            if rules.iter().all(|&(_f, t)| t != m) {
                // insert m into S
                s.insert(m);
            }
        }
    }

    sorted
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (143, 123));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (4924, 6085));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

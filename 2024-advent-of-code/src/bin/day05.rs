// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example05.txt");
const INPUT: &str = include_str!("input05.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn update_valid(update: &[&str], rules: &HashMap<&str, Vec<&str>>) -> bool {
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
    let (rules, updates) = parse(input);

    let bad_updates = updates.iter().filter(|update| update_valid(update, &rules));

    let part1 = bad_updates
        .map(|update| update[update.len() / 2].parse::<usize>().unwrap())
        .sum();

    (part1, 0)
}

fn parse(input: &str) -> (HashMap<&str, Vec<&str>>, Vec<Vec<&str>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut rule_map = HashMap::new();
    for rule in rules.lines() {
        let (left, right) = rule.split_once('|').unwrap();
        rule_map
            .entry(left)
            .and_modify(|deps: &mut Vec<&str>| deps.push(right))
            .or_insert_with(|| vec![right]);
    }

    let updates = updates
        .lines()
        .map(|line| line.split(",").collect())
        .collect();

    (rule_map, updates)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (143, 0));
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

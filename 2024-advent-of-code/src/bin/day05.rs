// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example05.txt");
const INPUT: &str = include_str!("input05.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (rules, updates) = parse(input);

    let mut sum = 0;

    'update: for update in updates {
        dbg!(&update);

        for i in 1..=update.len() {
            let set = &update[..i];
            let preceding = &set[..set.len() - 1];
            let target = set[set.len() - 1];
            println!("\t{preceding:?} {target:?}");
            if preceding.is_empty() {
                continue;
            }

            for p in preceding {
                // here, we have `target` coming after `p`
                // is there a rule that has `target` -> `p`?
                println!("\t\t{p}");

                if rules.get(target).is_some_and(|v| v.contains(&p)) {
                    println!("rule found for {target:?} -> {p:?}, INVALID");
                    continue 'update;
                } else {
                    println!("no rule for {target:?} -> {p:?}");
                }
            }
        }

        println!("update was ok");

        sum += update[update.len() / 2].parse::<usize>().unwrap();
    }

    (sum, 0)
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
    assert_eq!(solve(INPUT), (0, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

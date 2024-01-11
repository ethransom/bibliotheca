// #![feature(test)]

// extern crate test;

use std::collections::HashMap;
use std::ops::RangeInclusive;

const EXAMPLE: &str = include_str!("example19.txt");
const INPUT: &str = include_str!("input19.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u64, u64) {
    let (workflows, parts) = parse(input);

    let accepted_ratings = parts
        .iter()
        .filter(|&part| filter_part(&workflows, part))
        .map(|Part { props }| props.values().sum::<u64>())
        .sum();

    let all_possible_accepted = do_the_thing(&workflows);

    (accepted_ratings, all_possible_accepted)
}

const WORKFLOW_ACCEPT: &str = "A";
const WORKFLOW_REJECT: &str = "R";

#[derive(Debug, Clone)]
struct PartRange {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}

impl Default for PartRange {
    fn default() -> Self {
        PartRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}

impl PartRange {
    fn total_possible(&self) -> u64 {
        [&self.x, &self.m, &self.a, &self.s]
            .iter()
            .cloned()
            .map(|r| r.clone().count() as u64) // TODO: whyyyy double clone
            .product()
    }
}

fn limit_range(range: &RangeInclusive<u64>, op: char, value: u64) -> RangeInclusive<u64> {
    match op {
        '<' if *range.end() > value => *range.start()..=(value - 1),
        '>' if *range.start() < value => (value + 1)..=*range.end(),
        '<' | '>' => range.clone(),
        _ => panic!(),
    }
}

#[test]
fn test_limit_range() {
    // s<1351
    let range = 1..=4000;
    assert_eq!(limit_range(&range, '<', 1351), 1..=1350);
    // s>2770
    let range = 1..=4000;
    assert_eq!(limit_range(&range, '>', 2770), 2771..=4000);

    let range = 2771..=4000;
    assert_eq!(limit_range(&range, '>', 2000), 2771..=4000);
}

fn invert_range(range: &mut RangeInclusive<u64>, op: char, value: u64) {
    *range = match op {
        // everything NOT less than value => everything greater than or eq value
        '<' if *range.start() < value => value..=*range.end(),
        // everything NOT greater than value => everything less than or eq value
        '>' if *range.end() > value => *range.start()..=value,
        '<' | '>' => range.clone(), // TODO: lmao
        _ => panic!(),
    };
}

#[test]
fn test_invert_range() {
    let mut range = 1..=4000;
    invert_range(&mut range, '>', 2090);
    assert_eq!(range, 1..=2090);

    let mut range = 1..=2090;
    invert_range(&mut range, '>', 3000);
    assert_eq!(range, 1..=2090);
}

#[cfg(debug_assertions)]
fn rprint(str: String, depth: usize) {
    // return;
    for _ in 0..depth {
        print!("  ");
    }
    println!("{str}");
}

#[cfg(not(debug_assertions))]
fn rprint(_str: String, _depth: usize) {}

fn do_the_thing(workflows: &HashMap<&str, Vec<Rule>>) -> u64 {
    fn do_the_thing_2(
        workflows: &HashMap<&str, Vec<Rule>>,
        workflow: &str,
        mut range: PartRange,
        depth: usize,
    ) -> u64 {
        rprint(
            format!(
                "do_the_thing_2: {workflow} {range:?} {rules:?}",
                rules = workflows.get(workflow)
            ),
            depth,
        );
        if workflow == WORKFLOW_ACCEPT {
            let total = range.total_possible();
            rprint(format!("-> {total}"), depth + 1);
            return total;
        }
        if workflow == WORKFLOW_REJECT {
            return 0;
        }
        let rules = &workflows[workflow];

        let mut sum = 0;
        for rule in rules {
            let range = if let Some(cond) = rule.cond {
                rprint(format!("applying {cond:?} to {range:?}"), depth + 1);
                let Cond { key, op, value } = cond;
                let mut branch_range = range.clone();
                let (prop_branch_range, prop_range) = match key {
                    "x" => (&mut branch_range.x, &mut range.x),
                    "m" => (&mut branch_range.m, &mut range.m),
                    "a" => (&mut branch_range.a, &mut range.a),
                    "s" => (&mut branch_range.s, &mut range.s),
                    _ => panic!(),
                };
                *prop_branch_range = limit_range(prop_branch_range, op, value);
                invert_range(prop_range, op, value);
                rprint(
                    format!("got: {branch_range:?} for branch, {range:?} continuing"),
                    depth + 2,
                );
                branch_range
            } else {
                range.clone()
            };
            sum += do_the_thing_2(workflows, rule.dest, range, depth + 1); // TODO: wait do we multiply??
        }

        sum
    }

    do_the_thing_2(workflows, "in", PartRange::default(), 0)
}

#[derive(Debug, Copy, Clone)]
struct Cond<'a> {
    key: &'a str,
    op: char,
    value: u64,
}

#[derive(Debug)]
struct Rule<'a> {
    dest: &'a str,
    cond: Option<Cond<'a>>,
}

#[derive(Debug)]
struct Part<'a> {
    props: HashMap<&'a str, u64>,
}

fn filter_part(workflows: &HashMap<&str, Vec<Rule>>, Part { props }: &Part) -> bool {
    // println!("evaluating part {props:?}");
    let mut workflow = "in";
    loop {
        // println!("\tagainst workflow {workflow}");
        if workflow == WORKFLOW_ACCEPT {
            return true;
        }
        if workflow == WORKFLOW_REJECT {
            return false;
        }
        for &Rule { dest, cond } in &workflows[workflow] {
            // println!("\t\tfor rule {dest} {cond:?}");
            if let Some(Cond { key, op, value }) = cond {
                // println!(
                //     "\t\t\tcond: {cond:?}: {key} (={eval}) {op} {value}",
                //     eval = props[key]
                // );
                let op = match op {
                    '>' => u64::gt,
                    '<' => u64::lt,
                    _ => panic!(),
                };
                if op(&props[key], &value) {
                    workflow = dest;
                    break;
                }
            } else {
                // println!("\t\t\tno cond, straight to workflow {dest}");
                workflow = dest;
                break;
            }
        }
    }
}

fn parse(input: &str) -> (HashMap<&str, Vec<Rule>>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();

            let rules = rest.trim_end_matches('}');

            let rules = rules
                .split(',')
                .map(|rule| {
                    let (dest, cond) = if let Some((condition, dest)) = rule.split_once(':') {
                        let (key, op, value) = if let Some((key, value)) = condition.split_once('<')
                        {
                            (key, '<', value)
                        } else if let Some((key, value)) = condition.split_once('>') {
                            (key, '>', value)
                        } else {
                            panic!("unknown operation for condition: {condition}");
                        };
                        let value = value.parse().expect("couldn't parse");
                        (dest, Some(Cond { key, op, value }))
                    } else {
                        (rule, None)
                    };

                    Rule { dest, cond }
                })
                .collect();
            (name, rules)
        })
        .collect();

    let parts = parts
        .lines()
        .map(|line| {
            let line = line.trim_start_matches('{').trim_end_matches('}');

            let props = line
                .split(',')
                .map(|prop| {
                    let (key, value) = prop.split_once('=').unwrap();

                    (key, value.parse().unwrap())
                })
                .collect();

            Part { props }
        })
        .collect();

    (workflows, parts)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (19114, 167409079868000));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (348378, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

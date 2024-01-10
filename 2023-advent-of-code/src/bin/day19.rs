// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

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
        .filter(|Part { props }| {
            println!("evaluating part {props:?}");
            let mut workflow = "in";
            loop {
                println!("\tagainst workflow {workflow}");
                if workflow == "A" {
                    return true;
                }
                if workflow == "R" {
                    return false;
                }
                for &Rule { dest, cond } in &workflows[workflow] {
                    println!("\t\tfor rule {dest} {cond:?}");
                    if let Some(Cond { key, op, value }) = cond {
                        println!(
                            "\t\t\tcond: {cond:?}: {key} (={eval}) {op} {value}",
                            eval = props[key]
                        );
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
                        println!("\t\t\tno cond, straight to workflow {dest}");
                        workflow = dest;
                        break;
                    }
                }
            }
        })
        .map(|Part { props }| props.values().sum::<u64>())
        .sum();

    (accepted_ratings, 0)
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
    assert_eq!(solve(EXAMPLE), (19114, 0));
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

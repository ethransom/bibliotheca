#![feature(test)]

extern crate test;

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::collections::VecDeque;

const EXAMPLE: &str = include_str!("example20.txt");
const EXAMPLE_2: &str = include_str!("example20_2.txt");
const INPUT: &str = include_str!("input20.txt");

fn main() {
    dbg!(solve(EXAMPLE, 1, false));
    dbg!(solve(EXAMPLE_2, 3, false));
    dbg!(solve(INPUT, 1, true));
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum ModuleType<'a> {
    Output { activated_on: Option<usize> },
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { memory: HashMap<&'a str, bool> }, // conjunction function, what's your compunction
}

#[derive(Debug, Clone)]
struct Module<'a> {
    kind: ModuleType<'a>,
    // name: &'a str,
    destinations: Vec<&'a str>,
}

fn solve(input: &str, presses: usize, activate_rx: bool) -> (usize, usize) {
    let original_modules = parse(input);

    let mut modules = original_modules.clone();

    // dbg!(&modules);

    // modules.push(Module {
    //     kind: ModuleType::Button,
    //     destinations: vec![],
    // });

    let broadcaster = modules.get("broadcaster").unwrap();
    assert_eq!(broadcaster.kind, ModuleType::Broadcaster);

    let [mut high_pulses, mut low_pulses] = [0; 2];

    for press in 1..=presses {
        if press != 1 {
            // println!();
        }
        handle_press(&mut modules, press, &mut high_pulses, &mut low_pulses);
    }
    let total_sent = high_pulses * low_pulses;

    let mut min_for_output = 0;
    if activate_rx {
        let (name, rx_proxy) = modules
            .iter()
            .find(|(_name, module)| module.destinations.contains(&"rx"))
            .unwrap();
        assert!(matches!(rx_proxy.kind, ModuleType::Conjunction { .. }));
        println!("input to 'rx' is '{name}'");
        let stages = modules
            .iter()
            .filter(|(_name, module)| module.destinations.contains(name))
            .collect::<Vec<_>>();
        println!("inputs to '{name}' are {stages:?}");
        let stages = stages
            .iter()
            .map(|&(name, _module)| {
                let mut modules = modules.clone();
                let mut stage_output = 0;
                modules.insert(
                    name,
                    Module {
                        kind: ModuleType::Output { activated_on: None },
                        destinations: vec![],
                    },
                );

                for press in 1.. {
                    if let Some(output_module) = modules.get(name) {
                        if let ModuleType::Output {
                            activated_on: Some(activated_on),
                        } = output_module.kind
                        {
                            stage_output = activated_on;
                            break;
                        }
                    } else {
                        break;
                    }
                    handle_press(&mut modules, press, &mut high_pulses, &mut low_pulses);
                }

                stage_output
            })
            .collect::<Vec<_>>();
        println!("stages active on: {stages:?}");
        min_for_output = stages.into_iter().reduce(lcm).unwrap();
    }
    // for press in (presses + 1).. {
    //     if press % 100_000 == 0 {
    //         let press = press
    //             .to_string()
    //             .as_bytes()
    //             .rchunks(3)
    //             .rev()
    //             .map(std::str::from_utf8)
    //             .collect::<Result<Vec<&str>, _>>()
    //             .unwrap()
    //             .join(",");
    //         println!("{press}");
    //     }
    //     if let Some(output_module) = modules.get("rx") {
    //         if let ModuleType::Output {
    //             activated_on: Some(activated_on),
    //         } = output_module.kind
    //         {
    //             min_for_output = activated_on;
    //             break;
    //         }
    //     } else {
    //         break;
    //     }
    //     handle_press(&mut modules, press, &mut high_pulses, &mut low_pulses);
    // }

    (total_sent, min_for_output)
}

// borrowed from day 8
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    // for shame: copilot wrote this one
    // TODO: we do have this already oxidixed example:
    // https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Implementation
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn handle_press(
    modules: &mut HashMap<&str, Module>,
    press: usize,
    high_pulses: &mut usize,
    low_pulses: &mut usize,
) {
    let mut queue = VecDeque::<(&str, bool, &str)>::new();
    queue.push_back(("broadcaster", false, "button"));
    while let Some((name, pulse, from)) = queue.pop_front() {
        if pulse {
            *high_pulses += 1;
        } else {
            *low_pulses += 1;
        }
        // println!(
        //     "{from} {pulse}-> {name}",
        //     pulse = if pulse { "-high" } else { "-low" }
        // );
        let module = modules.get_mut(name).unwrap();
        match module.kind {
            ModuleType::Output {
                ref mut activated_on,
            } => {
                // println!(
                //     "{name} (Output) received {pulse}",
                //     pulse = if pulse { "-high" } else { "-low" }
                // );
                if !pulse {
                    *activated_on = Some(press);
                }
            }
            ModuleType::Broadcaster => {
                // When it receives a pulse, it sends the same pulse to all of its destination modules.
                for dest in &module.destinations {
                    queue.push_back((dest, pulse, name));
                }
            }
            ModuleType::Conjunction { ref mut memory } => {
                // remember the type of the most recent pulse received from each of their connected input
                // modules; they initially default to remembering a low pulse for each input. When a pulse
                // is received, the conjunction module first updates its memory for that input. Then, if it
                // remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                memory.insert(from, pulse);
                // println!("\tmemory: {memory:?}");
                let out = !memory.values().all(|&v| v);
                for dest in &module.destinations {
                    queue.push_back((dest, out, name));
                }
            }
            ModuleType::FlipFlop { ref mut state } => {
                // are either on or off; they are initially off. If a flip-flop module receives a high pulse,
                // it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it
                // flips between on and off. If it was off, it turns on and sends a high pulse. If it was on,
                // it turns off and sends a low pulse.
                if !pulse {
                    *state = !*state;
                    for dest in &module.destinations {
                        queue.push_back((dest, *state, name));
                    }
                }
            }
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Module> {
    let mut modules = input
        .lines()
        .map(|line| {
            let (mut module, destinations) = line.split_once(" -> ").unwrap();

            let kind = if module == "broadcaster" {
                ModuleType::Broadcaster
            } else if module.starts_with('%') {
                module = &module[1..];
                ModuleType::FlipFlop { state: false }
            } else if module.starts_with('&') {
                module = &module[1..];
                ModuleType::Conjunction {
                    memory: HashMap::default(),
                }
            } else {
                panic!()
            };

            let destinations = destinations.split(", ").collect();

            (
                module,
                Module {
                    kind,
                    // name: module,
                    destinations,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    // define implicit modules
    let defined = modules
        .values()
        .flat_map(|m| m.destinations.iter().cloned())
        .collect::<HashSet<_>>();
    let referenced = modules.keys().cloned().collect::<HashSet<_>>();
    let implicit = defined.difference(&referenced);
    for i in implicit {
        modules.insert(
            i,
            Module {
                kind: ModuleType::Output { activated_on: None },
                destinations: Default::default(),
            },
        );
    }

    // set conjunction default values
    let conjunction_names: Vec<_> = modules
        .iter()
        .filter_map(|(&name, m)| match &m.kind {
            ModuleType::Conjunction { memory: _memory } => Some(name),
            _ => None,
        })
        .collect();
    for name in conjunction_names {
        let sources: Vec<_> = modules
            .iter()
            .filter_map(|(&n, m)| {
                if m.destinations.contains(&name) {
                    Some(n)
                } else {
                    None
                }
            })
            .collect();
        if let ModuleType::Conjunction { ref mut memory } = modules.get_mut(name).unwrap().kind {
            for source in sources {
                memory.insert(source, false);
            }
        }
    }

    modules
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE, 1000, false), (32000000, 0));
}

#[test]
fn test_example_2() {
    assert_eq!(solve(EXAMPLE_2, 1000, false), (11687500, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT, 1000, true), (886701120, 0));
}

#[bench]
fn bench_handle_press(b: &mut test::Bencher) {
    let mut modules = parse(INPUT);
    b.iter(|| {
        let [mut high_pulses, mut low_pulses] = [0; 2];

        handle_press(&mut modules, 0, &mut high_pulses, &mut low_pulses);
    });
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

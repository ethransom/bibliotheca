#![feature(inline_const)]
// #![feature(test)]

// extern crate test;

use std::fmt::Debug;

const EXAMPLE: &str = include_str!("example15.txt");
const INPUT: &str = include_str!("input15.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &'static str) -> (usize, usize) {
    let sum = parse(input)
        .map(holiday_ascii_string_helper)
        .sum();

    let mut map = HolidayAsciiStringHelperManualArrangementProcedure::new();

    for step in parse(input) {
        if let Some((key, value)) = step.split_once('=') {
            map.insert(key, value.parse().unwrap());
        } else if let Some((key, _value)) = step.split_once('-') {
            map.remove(key);
        } else {
            panic!("invalid step");
        }
        println!("After \"{step}\":\n{map:?}\n");
    }

    let focusing_power = map.boxes.iter().enumerate().map(|(i, mut b)| {
        let mut sum = 0;
        let mut slot_no = 1;

        while let Some(slot) = b {
            sum += (i + 1) * (slot_no) * slot.value as usize;

            slot_no += 1;
            b = &slot.next;
        }

        sum
    }).sum();

    (sum, focusing_power)
}

struct HolidayAsciiStringHelperManualArrangementProcedure {
    boxes: [Option<Box<Slot>>; 256],
}

#[derive(Clone)]
struct Slot {
    key: &'static str,
    value: u8,
    next: Option<Box<Slot>>,
}

impl HolidayAsciiStringHelperManualArrangementProcedure {
    fn new() -> Self {
        HolidayAsciiStringHelperManualArrangementProcedure{
            boxes: [const { None }; 256],
        }
    }

    fn insert(&mut self, key: &'static str, value: u8) {
        // find if already in there
        let s = self.find(key);
        if let Some(slot) = s {
            slot.value = value;
        } else {
            // insert
            let mut b = &mut self.boxes[holiday_ascii_string_helper(key)];

            while let Some(slot) = b {
                b = &mut slot.next;
            }

            *b = Some(Box::new(Slot {
                key,
                value,
                next: None,
            }));
        }
    }

    fn find(&mut self, key: &str) -> Option<&mut Slot> {
        let mut b = &mut self.boxes[holiday_ascii_string_helper(key)];

        while let Some(slot) = b {
            if slot.key == key {
                return Some(slot);
            }
            b = &mut slot.next;
        }

        None
    }

    // fn find_in(mut b: &mut Option<Box<Slot>>, key: &'static str) -> Option<&mut Slot> {
    //     while let Some(mut slot) = b {
    //         if slot.key == key {
    //             return Some(&mut slot);
    //         }
    //         b = &mut slot.next;
    //     }
    //
    //     None
    // }

    fn remove(&mut self, key: &str) {
        let mut parent = &mut self.boxes[holiday_ascii_string_helper(key)];
        if let Some(child) = parent {
            if child.key == key {
                let next = std::mem::replace(&mut child.next, None);
                *parent = next; // also drops child?
                return;
            }
        } else {
            // None
            return;
        }
    }
}

impl Debug for HolidayAsciiStringHelperManualArrangementProcedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut have_printed = false;
        for (i, b) in self.boxes.iter().enumerate() {
            if let Some(slot) = b {
                if have_printed {
                    writeln!(f)?;
                }
                have_printed = true;
                write!(f, "Box {i}:")?;
                let mut slot = slot;
                loop {
                    write!(f, " [{key} {value}]", key = slot.key, value = slot.value)?;
                    if let Some(new_slot) = &slot.next {
                        slot = new_slot;
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

fn holiday_ascii_string_helper(input: &str) -> usize {
    input.as_bytes().iter().fold(0, |acc, &val| {
        ((acc + val as usize) * 17) % 256
    })
}

fn parse(input: &str) -> impl Iterator<Item=&str> {
    input.split(',')
}

#[test]
fn test_hashmap() {
    let mut map = HolidayAsciiStringHelperManualArrangementProcedure::new();

    // After "rn=1":
    map.insert("rn", 1);
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1]");

    // After "cm-":
    map.remove("cm-");
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1]");

    // After "qp=3":
    map.insert("qp", 3);
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1]\nBox 1: [qp 3]");

    // After "cm=2":
    map.insert("cm", 2);
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1] [cm 2]\nBox 1: [qp 3]");

    // After "qp-":
    map.remove("qp");
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1] [cm 2]");

    // After "pc=4":
    map.insert("pc", 4);
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1] [cm 2]\nBox 3: [pc 4]");

    // After "ot=9":
    map.insert("ot", 9);
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1] [cm 2]\nBox 3: [pc 4] [ot 9]");

    // After "ab=5":
    map.insert("ab", 5);
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1] [cm 2]\nBox 3: [pc 4] [ot 9] [ab 5]");

    // After "pc-":
    map.remove("pc");
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1] [cm 2]\nBox 3: [ot 9] [ab 5]");

    // After "pc=6":
    map.insert("pc", 6);
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1] [cm 2]\nBox 3: [ot 9] [ab 5] [pc 6]");

    // After "ot=7":
    map.insert("ot", 7);
    assert_eq!(format!("{map:?}"), "Box 0: [rn 1] [cm 2]\nBox 3: [ot 7] [ab 5] [pc 6]");
    //
    // // xtra
    // map.remove("ab");
    // assert_eq!(format!("{map:?}"), "Box 0: [rn 1] [cm 2]\nBox 3: [ot 7] [pc 6]");

}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1320, 145));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (516469, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

#![feature(test)]

use std::cmp::Ordering;

use itertools::Itertools;

extern crate test;

const EXAMPLE: &str = include_str!("example20.txt");
const INPUT: &str = include_str!("input20.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const DECRYPTION_KEY: i64 = 811589153;

fn solve(input: &str) -> (i64, i64) {
    let nums: Vec<(usize, i64)> = parse(input)
        .expect("couldn't parse input")
        .into_iter()
        .enumerate() // mark each with starting position
        .collect();

    let mixed_nums = {
        let mut nums = nums.clone();

        for i in 0..nums.len() {
            let (pos, &(_orig_pos, val)) = nums.iter().find_position(|p| p.0 == i).unwrap();
            shift_by(&mut nums, pos, val);
        }

        nums
    };

    let dekeyed_decamixed_nums = {
        let mut nums = nums.clone();

        // why why why
        for (_pos, val) in nums.iter_mut() {
            *val *= DECRYPTION_KEY;
        }

        for _ in 0..10 {
            for i in 0..nums.len() {
                let (pos, &(_orig_pos, val)) = nums.iter().find_position(|p| p.0 == i).unwrap();
                shift_by(&mut nums, pos, val);
            }
        }

        nums
    };

    let [message, dekeyed_decamixed_message] = [mixed_nums, dekeyed_decamixed_nums].map(|nums| {
        [1000, 2000, 3000]
            .map(|i| nums[(nums.iter().position(|p| p.1 == 0).unwrap() + i) % nums.len()].1)
            .iter()
            .sum()
    });

    (message, dekeyed_decamixed_message)
}

fn shift_by<E: std::fmt::Debug>(nums: &mut [E], pos: usize, val: i64) {
    let new_pos = if val > 0 {
        // RIGHT
        (pos + val as usize) % nums.len() + ((pos + val as usize) / nums.len() % nums.len())
    } else {
        // LEFT
        nums.len() - 1 - ((pos as i64 + val) % nums.len() as i64).unsigned_abs() as usize
    };

    match new_pos.cmp(&pos) {
        Ordering::Greater => nums[pos..=new_pos].rotate_left(1),
        Ordering::Less => nums[new_pos..=pos].rotate_right(1),
        Ordering::Equal => (),
    };
}

// Left here as a monument to the triumph of overengineering (not like that other function later in the file.)
// It worked great, right up until it didn't.
fn _too_slow_shift_by<E>(nums: &mut [E], mut pos: usize, val: i64) {
    // dbg!(nums.iter().map(|p| p.1).join(", "), pos, val);
    if val > 0 {
        // RIGHT
        let mut val = val as usize;
        while val > 0 {
            if pos == nums.len() - 1 {
                nums.rotate_right(1);
                pos = 0;
                // do not subtract from val for some godforsaken reason I really do not understand
            }
            nums.swap(pos, pos + 1);
            pos += 1;
            val -= 1;
        }
        if pos == nums.len() - 1 {
            nums.rotate_right(1);
        }
    } else {
        // LEFT
        let mut val = val.unsigned_abs() as usize;
        while val > 0 {
            if pos == 0 {
                nums.rotate_left(1);
                pos = nums.len() - 1;
                // do not subtract from val for some godforsaken reason I really do not understand
            }
            nums.swap(pos, pos - 1);
            pos -= 1;
            val -= 1;
        }
        if pos == 0 {
            nums.rotate_left(1);
        }
    }

    // dbg!(nums.iter().map(|p| p.1).join(", "));
}

// left here as a monument to overengineering
// YAGNI, always. Expecially when you think you do need it.
fn _single_shot_shift_by(nums: &mut [(usize, i32)], pos: usize, val: i32) {
    dbg!(nums.iter().map(|p| p.1).join(", "), pos, val);

    if val >= 0 {
        // RIGHT
        let val = val as usize;
        if val + pos > nums.len() {
            dbg!(nums.iter().map(|p| p.1).join(", "));
            // will wrap around the end of array
            let len = nums.len();
            let mut val = val + 1;
            let mut pos = pos;
            while val + pos > nums.len() {
                // rotate pos to end
                nums[pos..].rotate_right(len - 1 - pos);
                dbg!(nums.iter().map(|p| p.1).join(", "));

                // rotate pos to start
                nums.rotate_right(1);
                dbg!(nums.iter().map(|p| p.1).join(", "));

                val -= len - 1 - pos;
                pos = 0;
            }
            dbg!(nums.iter().map(|p| p.1).join(", "), pos, val);

            // rotate from start to final
            // nums[0..val - (len - 1 - pos)].rotate_right(val - (len - 1 - pos) - 1);
            nums[pos..=pos + val].rotate_right(val);
        } else {
            nums[pos..=pos + val].rotate_right(val);
        }
    } else {
        // LEFT
        let val = val.unsigned_abs() as usize + 1;
        if val > pos {
            // will wrap around the start of array
            let len = nums.len();
            nums[0..=pos].rotate_left(pos);
            nums.rotate_left(1);
            nums[len - (val - pos)..].rotate_left(val - pos - 1);
        } else {
            nums[pos - val..=pos].rotate_left(val);
        }
    }
}

fn parse(input: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[test]
fn test_shift_part_one() {
    let mut nums: Vec<i32> = vec![1, 2, -3, 3, -2, 0, 4];

    // 1 moves between 2 and -3:
    shift_by(&mut nums, 0, 1);
    assert_eq!(&nums, &vec![2, 1, -3, 3, -2, 0, 4]);

    // 2 moves between -3 and 3:
    shift_by(&mut nums, 0, 2);
    assert_eq!(&nums, &vec![1, -3, 2, 3, -2, 0, 4]);

    // -3 moves between -2 and 0:
    shift_by(&mut nums, 1, -3);
    assert_eq!(&nums, &vec![1, 2, 3, -2, -3, 0, 4]);

    // 3 moves between 0 and 4:
    shift_by(&mut nums, 2, 3);
    assert_eq!(&nums, &vec![1, 2, -2, -3, 0, 3, 4]);

    // -2 moves between 4 and 1:
    shift_by(&mut nums, 2, -2);
    assert_eq!(&nums, &vec![1, 2, -3, 0, 3, 4, -2]);

    // 0 does not move:
    shift_by(&mut nums, 3, 0);
    assert_eq!(&nums, &vec![1, 2, -3, 0, 3, 4, -2]);

    // 4 moves between -3 and 0:
    shift_by(&mut nums, 5, 4);
    assert_eq!(&nums, &vec![1, 2, -3, 4, 0, 3, -2]);
}

#[test]
fn test_shift_part_two() {
    // Initial arrangement:
    let mut nums = vec![811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612];

    After 1 round of mixing:
    0, -2434767459, 3246356612, -1623178306, 2434767459, 1623178306, 811589153

    After 2 rounds of mixing:
    0, 2434767459, 1623178306, 3246356612, -2434767459, -1623178306, 811589153

    After 3 rounds of mixing:
    0, 811589153, 2434767459, 3246356612, 1623178306, -1623178306, -2434767459

    After 4 rounds of mixing:
    0, 1623178306, -2434767459, 811589153, 2434767459, 3246356612, -1623178306

    After 5 rounds of mixing:
    0, 811589153, -1623178306, 1623178306, -2434767459, 3246356612, 2434767459

    After 6 rounds of mixing:
    0, 811589153, -1623178306, 3246356612, -2434767459, 1623178306, 2434767459

    After 7 rounds of mixing:
    0, -2434767459, 2434767459, 1623178306, -1623178306, 811589153, 3246356612

    After 8 rounds of mixing:
    0, 1623178306, 3246356612, 811589153, -2434767459, 2434767459, -1623178306

    After 9 rounds of mixing:
    0, 811589153, 1623178306, -2434767459, 3246356612, 2434767459, -1623178306

    After 10 rounds of mixing:
    0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (3, 1_623_178_306));
}

#[test]
fn test_double_wrap() {
    let mut nums = vec![1, 2, -3, 0, 3, 4, -2];
    shift_by(&mut nums, 5, 8);
    assert_eq!(&nums, &vec![1, 4, 2, -3, 0, 3, -2]);
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (7_225, 0));
    });
}

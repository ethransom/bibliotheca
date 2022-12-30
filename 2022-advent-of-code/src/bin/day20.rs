#![feature(test)]

use itertools::Itertools;

extern crate test;

const EXAMPLE: &str = include_str!("example20.txt");
const INPUT: &str = include_str!("input20.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (i32, usize) {
    let mut nums: Vec<(usize, i32)> = parse(input)
        .expect("couldn't parse input")
        .into_iter()
        .enumerate() // mark each with starting position
        .collect();

    for i in 0..nums.len() {
        let (pos, &(_orig_pos, val)) = nums.iter().find_position(|p| p.0 == i).unwrap();
        shift_by(&mut nums, pos, val);
    }

    let message = [1000, 2000, 3000]
        .map(|i| nums[(nums.iter().position(|p| p.1 == 0).unwrap() + i) % nums.len()].1)
        .iter()
        .sum();

    (message, 0)
}

fn shift_by(nums: &mut [(usize, i32)], mut pos: usize, val: i32) {
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

fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[test]
fn test_example() {
    fn cmp(a: &[(usize, i32)], b: &[i32]) {
        assert_eq!(a.iter().map(|p| p.1).collect::<Vec<i32>>(), b);
    }

    let mut nums: Vec<(usize, i32)> = vec![1, 2, -3, 3, -2, 0, 4]
        .into_iter()
        .enumerate()
        .collect();

    // 1 moves between 2 and -3:
    shift_by(&mut nums, 0, 1);
    cmp(&nums, &vec![2, 1, -3, 3, -2, 0, 4]);

    // 2 moves between -3 and 3:
    shift_by(&mut nums, 0, 2);
    cmp(&nums, &vec![1, -3, 2, 3, -2, 0, 4]);

    // -3 moves between -2 and 0:
    shift_by(&mut nums, 1, -3);
    cmp(&nums, &vec![1, 2, 3, -2, -3, 0, 4]);

    // 3 moves between 0 and 4:
    shift_by(&mut nums, 2, 3);
    cmp(&nums, &vec![1, 2, -2, -3, 0, 3, 4]);

    // -2 moves between 4 and 1:
    shift_by(&mut nums, 2, -2);
    cmp(&nums, &vec![1, 2, -3, 0, 3, 4, -2]);

    // 0 does not move:
    shift_by(&mut nums, 3, 0);
    cmp(&nums, &vec![1, 2, -3, 0, 3, 4, -2]);

    // 4 moves between -3 and 0:
    shift_by(&mut nums, 5, 4);
    cmp(&nums, &vec![1, 2, -3, 4, 0, 3, -2]);

    assert_eq!(solve(EXAMPLE), (3, 0));
}

#[test]
fn test_double_wrap() {
    let mut nums: Vec<(usize, i32)> = vec![1, 2, -3, 0, 3, 4, -2]
        .into_iter()
        .enumerate()
        .collect();
    shift_by(&mut nums, 5, 8);
    assert_eq!(
        &nums.iter().map(|p| p.1).collect::<Vec<i32>>(),
        &vec![1, 4, 2, -3, 0, 3, -2]
    );
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (7_225, 0));
    });
}

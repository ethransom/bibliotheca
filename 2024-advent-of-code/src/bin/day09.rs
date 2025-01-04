// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example09.txt");
const INPUT: &str = include_str!("input09.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let blocks = parse(input);

    let mut disk = Vec::<Option<usize>>::new();
    let mut id = 0;
    for (i, len) in blocks.iter().enumerate() {
        let block = i % 2 == 0;
        if block {
            for _ in 0..*len {
                disk.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..*len {
                disk.push(None);
            }
        }
    }

    (part1(disk.clone()), 0)
}

fn part1(mut disk: Vec<Option<usize>>) -> usize {
    let mut i = 0;
    let mut len = disk.len();
    loop {
        if i >= len {
            break;
        }
        if disk[i].is_none() {
            while disk[len - 1].is_none() {
                len -= 1;
                if i >= len - 1 {
                    break;
                }
            }

            disk.swap(i, len - 1);
        }
        // for v in disk.iter() {
        //     let c = if v.is_some() { 'X' } else { '.' };
        //     print!("{c}");
        // }
        // println!("");
        i += 1;
    }

    checksum(&disk)
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, c)| c.map(|c| i * c))
        .sum()
}

fn parse(input: &str) -> Vec<u8> {
    input.trim().bytes().map(btoi).collect::<Vec<_>>()
}

fn btoi(b: u8) -> u8 {
    b - b'0'
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1928, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (6607511583593, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

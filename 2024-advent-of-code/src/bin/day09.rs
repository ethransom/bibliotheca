#![feature(exact_size_is_empty)]
// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example09.txt");
const INPUT: &str = include_str!("input09.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let input = parse(input);

    let mut disk = Vec::<Option<usize>>::new();
    let mut id = 0;
    for (i, len) in input.iter().enumerate() {
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

    (part1(disk.clone()), part2(input))
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
        // print(&disk);
        i += 1;
    }

    checksum(&disk)
}

fn part2(sector_lengths: Vec<u8>) -> usize {
    let mut free_list: Vec<(usize, usize)> = vec![];
    let mut sector_list: Vec<(usize, usize, usize)> = vec![];
    let mut cursor = 0;
    let mut id = 0;
    let mut sector_lengths = sector_lengths.iter();
    while !sector_lengths.is_empty() {
        // FILE
        let &len = sector_lengths.next().unwrap();

        let len = len as usize;

        sector_list.push((cursor, len, id as usize));

        cursor += len;
        id += 1;

        // FREE
        let Some(&len) = sector_lengths.next() else {
            continue;
        };

        let len = len as usize;

        free_list.push((cursor, len));

        cursor += len;
    }

    fn print(sector_list: &Vec<(usize, usize, usize)>) {
        let mut cursor = 0;
        for &sector in sector_list {
            let (pos, len, id) = sector;

            while cursor < pos {
                print!(".");
                cursor += 1;
            }

            let id = id.to_string().chars().last().unwrap();
            for _i in 0..len {
                cursor += 1;
                print!("{id}");
            }
        }
        println!();
    }

    println!("BEFORE:");
    print(&sector_list);

    for sector in sector_list.iter_mut().rev() {
        let (pos, len, _id) = sector;
        let Some((index, (free_pos, free_len))) = free_list
            .iter_mut()
            .enumerate()
            .find(|(_i, (_pos, free_len))| *free_len >= *len)
        else {
            // no space found
            continue;
        };
        *pos = *free_pos;
        // move up
        *free_pos += *len;
        // shrink
        *free_len -= *len;
        if *free_len == 0 {
            // NOTE: technically we don't need to do this but it is good hygine I suppose
            free_list.remove(index);
        }
    }
    sector_list.sort(); // TODO: unecessary?? probably not
    print(&sector_list);
    println!("00992111777.44.333....5555.6666.....8888..");

    fn checksum(sector_list: &Vec<(usize, usize, usize)>) -> usize {
        let mut checksum = 0;
        for &sector in sector_list {
            let (pos, len, id) = sector;

            for i in 0..len {
                checksum += (pos + i) * id;
            }
        }
        checksum
    }

    checksum(&sector_list)
}

fn print(disk: &[Option<usize>]) {
    for v in disk.iter() {
        let c = if let Some(v) = v {
            v.to_string().chars().last().unwrap()
        } else {
            '.'
        };
        print!("{c}");
    }
    println!();
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
    assert_eq!(solve(EXAMPLE), (1928, 2858));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (6607511583593, 8868021291269));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

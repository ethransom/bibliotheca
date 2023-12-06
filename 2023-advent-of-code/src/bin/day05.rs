#![feature(test)]
#![feature(array_chunks)]

extern crate test;

const EXAMPLE: &str = include_str!("example05.txt");
const INPUT: &str = include_str!("input05.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u64, u64) {
    let (seeds, maps) = parse(input);

    let mut part1 = u64::MAX;

    for &seed in &seeds {
        let mut resource = "seed";
        let mut id = seed;
        while resource != "location" {
            let map = maps.iter().find(|map| map.src == resource).unwrap();
            id = map.map(id);
            resource = map.dst;
        }

        #[cfg(debug_assertions)]
        println!("{seed} mapped to location: {id}");

        part1 = part1.min(id);
    }

    let mut part2 = u64::MAX;
    for (c, [start, size]) in seeds.array_chunks::<2>().enumerate() {
        println!("chunk {c}: {start}..{end}", end = start + size);
        for i in 0..=*size {
            let seed = start + i;
            let mut resource = "seed";
            let mut id = seed;
            while resource != "location" {
                let map = maps.iter().find(|map| map.src == resource).unwrap();
                id = map.map(id);
                resource = map.dst;
            }

            #[cfg(debug_assertions)]
            println!("{seed} mapped to location: {id}");

            part2 = part2.min(id);
        }
    }

    (part1, part2)
}

#[derive(Debug)]
struct Map<'a> {
    src: &'a str,
    dst: &'a str,
    ranges: Vec<(u64, u64, u64)>,
}

impl Map<'_> {
    fn map(&self, id: u64) -> u64 {
        for (dst_start, src_start, size) in &self.ranges {
            if id >= *src_start && id < *src_start + *size {
                return id - src_start + dst_start;
            }
        }

        // implicit mapping
        id
    }
}

#[test]
fn test_map_map() {
    let map = Map {
        src: "seed",
        dst: "location",
        ranges: vec![(50, 98, 2), (52, 50, 48)],
    };

    // seed -> soil
    assert_eq!(map.map(0), 0);
    assert_eq!(map.map(1), 1);
    // ...
    assert_eq!(map.map(48), 48);
    assert_eq!(map.map(49), 49);
    assert_eq!(map.map(50), 52);
    assert_eq!(map.map(51), 53);
    // ...
    assert_eq!(map.map(96), 98);
    assert_eq!(map.map(97), 99);
    assert_eq!(map.map(98), 50);
    assert_eq!(map.map(99), 51);
}

fn parse(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) // "seeds: "
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let maps = sections
        .map(|section| {
            let (header, ranges) = section.split_once(" map:\n").unwrap();

            let (src, dst) = header.split_once("-to-").unwrap();

            let ranges = ranges
                .lines()
                .map(|line| {
                    let range: [u64; 3] = line
                        .split_whitespace()
                        .map(str::parse)
                        .collect::<Result<Vec<u64>, _>>()
                        .unwrap()
                        .try_into()
                        .expect("expected three numbers");

                    range.try_into().unwrap()
                })
                .collect();

            Map { src, dst, ranges }
        })
        .collect();

    (seeds, maps)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (35, 46));
}

#[test]
fn test_input() {
    // too slow lol
    // assert_eq!(solve(INPUT), (318728750, 37384986));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (318728750, 37384986));
    });
}

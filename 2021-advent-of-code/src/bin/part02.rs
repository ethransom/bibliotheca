#![feature(test)]

const EXAMPLE: &[u8] = include_bytes!("example02.txt");
const INPUT: &[u8] = include_bytes!("input02.txt");

fn main() {
    println!("Example:");
    let (one, two) = solve(EXAMPLE);
    println!("\tpart1: {}\n\tpart2: {}", one, two);
    println!("Input:");
    let (one, two) = solve(INPUT);
    println!("\tpart1: {}\n\tpart2: {}", one, two);
}

#[derive(Debug)]
enum Move {
    Forward,
    Up,
    Down,
}

fn solve(input: &[u8]) -> (u64, u64) {
    let commands = parse(input);

    // println!("{:?}", commands);

    (part1(&commands), part2(&commands))
}

fn parse(input: &[u8]) -> Vec<(Move, u64)> {
    let mut commands: Vec<(Move, u64)> = Vec::new();
    input.split(|b| *b == '\n' as u8).for_each(|line| {
        let split = line
            .iter()
            .position(|&r| r == ' ' as u8)
            .expect("expected space as delimiter");
        let dir = match &line[0..split] {
            b"forward" => Move::Forward,
            b"up" => Move::Up,
            b"down" => Move::Down,
            _ => panic!("unknown command"),
        };
        let amt = std::str::from_utf8(&line[split + 1..])
            .expect("bad input file!")
            .parse::<u64>()
            .expect("couldn't parse");
        commands.push((dir, amt));
    });
    commands
}

fn part1(commands: &Vec<(Move, u64)>) -> u64 {
    let (depth, distance) = commands
        .iter()
        .fold((0u64, 0u64), |(x, y), command| match command {
            (Move::Forward, a) => (x + a, y),
            (Move::Up, a) => (x, y - a),
            (Move::Down, a) => (x, y + a),
        });

    depth * distance
}

fn part2(commands: &Vec<(Move, u64)>) -> u64 {
    let (depth, distance, _aim) = commands.iter().fold(
        (0u64, 0u64, 0u64),
        |(depth, dist, aim), command| match command {
            (Move::Forward, x) => (depth + x, dist + aim * x, aim),
            (Move::Up, x) => (depth, dist, aim - x),
            (Move::Down, x) => (depth, dist, aim + x),
        },
    );

    depth * distance
}

#[test]
fn it_handles_the_example_input() {
    assert_eq!(solve(EXAMPLE), (150, 900));
}

extern crate test;

#[bench]
fn bench_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (1990000, 1975421260));
    });
}

#[bench]
fn bench_parse_00_original(b: &mut test::Bencher) {
    b.iter(|| {
        let mut commands: Vec<(Move, u64)> = Vec::new();
        std::str::from_utf8(INPUT)
            .expect("bad input file!")
            .lines()
            .for_each(|line| {
                let parts: Vec<&str> = line.split(' ').take(2).collect();
                let dir = match parts[0] {
                    "forward" => Move::Forward,
                    "up" => Move::Up,
                    "down" => Move::Down,
                    _ => panic!("unknown command"),
                };
                let amt = parts[1].parse::<u64>().expect("couldn't parse");
                commands.push((dir, amt));
            });
        commands
    });
}

#[bench]
fn bench_parse_01_bytes(b: &mut test::Bencher) {
    b.iter(|| {
        let mut commands: Vec<(Move, u64)> = Vec::new();
        INPUT.split(|b| *b == '\n' as u8).for_each(|line| {
            let split = line
                .iter()
                .position(|&r| r == ' ' as u8)
                .expect("expected space as delimiter");
            let dir = match &line[0..split] {
                b"forward" => Move::Forward,
                b"up" => Move::Up,
                b"down" => Move::Down,
                _ => panic!("unknown command"),
            };
            let amt = std::str::from_utf8(&line[split + 1..])
                .expect("bad input file!")
                .parse::<u64>()
                .expect("couldn't parse");
            commands.push((dir, amt));
        });
        commands
    });
}

#[bench]
fn bench_parse_02_bytes_map(b: &mut test::Bencher) {
    // largely a failed experiment, seems slightly slower
    b.iter(|| {
        INPUT
            .split(|b| *b == '\n' as u8)
            .map(|line| {
                let split = line
                    .iter()
                    .position(|&r| r == ' ' as u8)
                    .expect("expected space as delimiter");
                let dir = match &line[0..split] {
                    b"forward" => Move::Forward,
                    b"up" => Move::Up,
                    b"down" => Move::Down,
                    _ => panic!("unknown command"),
                };
                let amt = std::str::from_utf8(&line[split + 1..])
                    .expect("bad input file!")
                    .parse::<u64>()
                    .expect("couldn't parse");
                (dir, amt)
            })
            .collect::<Vec<(Move, u64)>>()
    });
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    let commands = parse(INPUT);
    b.iter(|| (part1(&commands), part2(&commands)));
}

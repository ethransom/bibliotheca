const EXAMPLE: &[u8] = include_bytes!("example02.txt");
const INPUT: &[u8] = include_bytes!("input02.txt");

#[derive(Debug)]
enum Move {
    Forward,
    Up,
    Down,
}

fn main() {
    println!("Example:");
    solve(EXAMPLE);
    println!("Input:");
    solve(INPUT);
}

fn solve(input: &[u8]) {
    let mut commands: Vec<(Move, u64)> = Vec::new();
    std::str::from_utf8(input)
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

    // println!("{:?}", commands);

    let (depth, distance) = commands
        .iter()
        .fold((0u64, 0u64), |(x, y), command| match command {
            (Move::Forward, a) => (x + a, y),
            (Move::Up, a) => (x, y - a),
            (Move::Down, a) => (x, y + a),
        });

    println!("{:?}", depth * distance);

    let (depth, distance, _aim) = commands.iter().fold(
        (0u64, 0u64, 0u64),
        |(depth, dist, aim), command| match command {
            (Move::Forward, x) => (depth + x, dist + aim * x, aim),
            (Move::Up, x) => (depth, dist, aim - x),
            (Move::Down, x) => (depth, dist, aim + x),
        },
    );

    println!("{:?}", depth * distance);
}

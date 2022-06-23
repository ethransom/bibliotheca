#![feature(test)]

extern crate test;

use std::fmt::Formatter;

const EXAMPLE: &str = include_str!("example23.txt");
const INPUT: &str = include_str!("input23.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let rooms = parse(input);

    println!("{}", rooms);

    (0, 0)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Burrow {
    hallway: [Option<char>; 5],
    rooms: [[Option<char>; 2]; 4],
}

impl std::fmt::Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#############
#.{}.{}.{}.{}.{}.#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #########",
            self.hallway[0].unwrap_or('.'),
            self.hallway[1].unwrap_or('.'),
            self.hallway[2].unwrap_or('.'),
            self.hallway[3].unwrap_or('.'),
            self.hallway[4].unwrap_or('.'),
            self.rooms[0][0].unwrap_or('.'),
            self.rooms[1][0].unwrap_or('.'),
            self.rooms[2][0].unwrap_or('.'),
            self.rooms[3][0].unwrap_or('.'),
            self.rooms[0][1].unwrap_or('.'),
            self.rooms[1][1].unwrap_or('.'),
            self.rooms[2][1].unwrap_or('.'),
            self.rooms[3][1].unwrap_or('.'),
        )
    }
}

fn parse(input: &str) -> Burrow {
    let lines = input.lines().collect::<Vec<&str>>();

    let rooms = [3, 5, 7, 9].map(|col| {
        [2, 3].map(|row| {
            let c = lines[row].chars().nth(col).unwrap();
            if c == '.' {
                None
            } else {
                Some(c)
            }
        })
    });

    let hallway = [2, 4, 6, 8, 10].map(|spot| {
        let c = lines[1].chars().nth(spot).unwrap();
        if c == '.' {
            None
        } else {
            Some(c)
        }
    });

    Burrow { hallway, rooms }
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(
            "#############
#...B.......#
###B#C#.#D###
  #A#D#C#A#
  #########"
        ),
        Burrow {
            hallway: [None, Some('B'), None, None, None],
            rooms: [
                [Some('B'), Some('A')],
                [Some('C'), Some('D')],
                [None, Some('C')],
                [Some('D'), Some('A')]
            ],
        }
    );
}

#[test]
fn test_get_moves() {
    assert_eq!(get_moves(&parse(EXAMPLE)).len(), 20);
    assert_eq!(
        get_moves(&parse(
            "#############
#...B.......#
###B#C#.#D###
  #A#D#C#A#
  #########"
        ))
        .len(),
        7
    );
    assert_eq!(
        get_moves(&parse(
            "#############
#...B.......#
###B#.#C#D###
  #A#D#C#A#
  #########"
        ))
        .len(),
        8
    );
    assert_eq!(
        get_moves(&parse(
            "#############
#.....D.....#
###B#.#C#D###
  #A#B#C#A#
  #########"
        ))
        .len(),
        5
    );
    assert_eq!(
        get_moves(&parse(
            "#############
#.....D.....#
###.#B#C#D###
  #A#B#C#A#
  #########"
        ))
        .len(),
        2
    );
    assert_eq!(
        get_moves(&parse(
            "#############
#.....D.D.A.#
###.#B#C#.###
  #A#B#C#.#
  #########"
        )),
        vec![parse(
            "#############
#.......D.A.#
###.#B#C#.###
  #A#B#C#D#
  #########"
        )]
    );
    assert_eq!(
        get_moves(&parse(
            "#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#
  #########"
        )),
        vec![parse(
            "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########"
        )]
    );
    assert_eq!(
        get_moves(&parse(
            "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########"
        )),
        vec![]
    );
}

fn get_moves(burrow: &Burrow) -> Vec<Burrow> {
    let mut output = vec![];

    // HALLWAY INTO ROOMS
    for hallway in 0..burrow.hallway.len() {
        if let Some(hallpod) = burrow.hallway[hallway] {
            for room in 0..burrow.rooms.len() {
                // cannot enter non-destination room
                if (hallpod as u8 - b'A') != room as u8 {
                    continue;
                }
                if let None = burrow.rooms[room][1] {
                    let mut burrow: Burrow = burrow.clone();
                    burrow.rooms[room][1] = Some(hallpod);
                    burrow.hallway[hallway] = None;
                    output.push(burrow);
                    break;
                }
                if let None = burrow.rooms[room][0] {
                    let mut burrow: Burrow = burrow.clone();
                    burrow.rooms[room][0] = Some(hallpod);
                    burrow.hallway[hallway] = None;
                    output.push(burrow);
                    break;
                }
            }
            break;
        }
    }

    // ROOM DIRECTLY INTO ROOM
    for src in 0..burrow.rooms.len() {
        if let Some(roompod) = burrow.rooms[src][0] {
            for dst in 0..burrow.rooms.len() {
                if src == dst {
                    continue;
                }
                // do not enter a room that has an unsatisfied amphipod below
                if (roompod as u8 - b'A') != dst as u8 {
                    continue;
                }
                let path_blocked = if src > dst {
                    dst..=src
                } else {
                    (src + 1)..=dst
                }
                .find_map(|i| burrow.hallway[i]);
                if let Some(_) = path_blocked {
                    continue;
                }
                if let None = burrow.rooms[dst][0] {
                    let mut burrow: Burrow = burrow.clone();
                    burrow.rooms[dst][0] = Some(roompod);
                    burrow.rooms[src][0] = None;
                    output.push(burrow);
                }
            }
            break;
        }
    }

    // ROOM INTO HALLWAY
    for room in 0..burrow.rooms.len() {
        for depth in 0..2 {
            if let Some(pod) = burrow.rooms[room][depth] {
                if (pod as u8 - b'A') == room as u8 {
                    if depth == 1 {
                        // pod is correctly placed, cannot move
                        break;
                    } else if let Some(underpod) = burrow.rooms[room][1] {
                        if (underpod as u8 - b'A') == room as u8 {
                            // pod, and the pod below it are placed, cannot move
                            break;
                        }
                    }
                }
                for hallway in 0..burrow.hallway.len() {
                    let path_blocked = (if hallway > room {
                        room + 1..=hallway
                    } else {
                        hallway..=room
                    })
                    .find_map(|i| burrow.hallway[i]);
                    if let Some(_) = path_blocked {
                        continue;
                    }
                    let mut burrow: Burrow = burrow.clone();
                    burrow.hallway[hallway] = burrow.rooms[room][depth];
                    burrow.rooms[room][depth] = None;
                    output.push(burrow);
                }
                break;
            }
        }
    }

    println!("moves for input:\n{}\n------------------------\n", burrow);
    for b in &output {
        println!("{}\n", b);
    }

    output
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}

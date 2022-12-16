#![feature(test)]

use std::iter::Peekable;

use anyhow::{anyhow, Context, Result};

extern crate test;

const EXAMPLE: &str = include_str!("example13.txt");
const INPUT: &str = include_str!("input13.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let pairs = parse(input).expect("could not parse input");

    let indexes_of_ordered = pairs
        .iter()
        .enumerate()
        .map(|(index, (left, right))| {
            if left.partial_cmp(right).unwrap() == std::cmp::Ordering::Less {
                index + 1
            } else {
                0
            }
        })
        .sum();

    let divider_packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
    ];

    let mut all_packets = pairs
        .iter()
        .flat_map(|(left, right)| vec![left, right])
        .chain(divider_packets.iter())
        .collect::<Vec<_>>();

    all_packets.sort();

    for packet in all_packets.iter() {
        println!("{}", packet.to_string());
    }

    let decoder_key = all_packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| divider_packets.contains(packet))
        .map(|(index, _)| index + 1)
        .product();

    (indexes_of_ordered, decoder_key)
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        match self {
            Packet::Int(int) => int.to_string(),
            Packet::List(list) => {
                let inner = list
                    .iter()
                    .map(|packet| packet.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("[{}]", inner)
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();

                loop {
                    match (a.next(), b.next()) {
                        (Some(a), Some(b)) => match a.cmp(b) {
                            std::cmp::Ordering::Equal => continue,
                            ordering => return ordering,
                        },
                        (Some(_), None) => return std::cmp::Ordering::Greater,
                        (None, Some(_)) => return std::cmp::Ordering::Less,
                        (None, None) => return std::cmp::Ordering::Equal,
                    }
                }
            }
            (Packet::Int(int), list /* must be list */) => {
                Packet::List(vec![Packet::Int(*int)]).cmp(list)
            }
            (list /* must be list */, Packet::Int(int)) => {
                list.cmp(&Packet::List(vec![Packet::Int(*int)]))
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Result<Vec<(Packet, Packet)>> {
    input
        .split("\n\n")
        .map(|pair| {
            let (a, b) = pair
                .split_once('\n')
                .context("requires two packets per block")?;

            Ok((
                parse_packet(&mut Lexer::from(a.as_bytes()).peekable())?
                    .context("expected packet")?,
                parse_packet(&mut Lexer::from(b.as_bytes()).peekable())?
                    .context("expected packet")?,
            ))
        })
        .collect()
}

fn parse_packet(lexer: &mut Peekable<Lexer>) -> Result<Option<Packet>> {
    if let Some(Token::OpenBracket) = lexer.peek() {
        lexer.next();
    } else {
        return Ok(None);
    };

    let mut list = vec![];

    loop {
        if let Some(packet) = parse_packet(lexer)? {
            list.push(packet);
            continue;
        } else {
            match lexer.next() {
                Some(Token::Int(int)) => list.push(Packet::Int(int)),
                Some(Token::Comma) => continue,
                Some(Token::CloseBracket) => return Ok(Some(Packet::List(list))),
                _ => return Err(anyhow!("expected , or ] or digit")),
            }
        }

        if let Some(Token::Comma) = lexer.peek() {
            lexer.next();
            continue;
        } else if let Some(Token::CloseBracket) = lexer.next() {
            return Ok(Some(Packet::List(list)));
        } else {
            return Err(anyhow!("expected , or ]"));
        }
    }
}

#[derive(Debug)]
enum Token {
    Int(u8),
    OpenBracket,
    CloseBracket,
    Comma,
}

struct Lexer<'a> {
    input: &'a [u8],
}

impl<'a> Lexer<'a> {
    fn from(input: &'a [u8]) -> Self {
        Self { input }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(input) = self.input.strip_prefix(b"[") {
            self.input = input;
            Some(Token::OpenBracket)
        } else if let Some(input) = self.input.strip_prefix(b"]") {
            self.input = input;
            Some(Token::CloseBracket)
        } else if let Some(input) = self.input.strip_prefix(b",") {
            self.input = input;
            Some(Token::Comma)
        } else {
            parse_int(&mut self.input).map(Token::Int)
        }
    }
}

fn parse_int(input: &mut &[u8]) -> Option<u8> {
    let original = *input;
    while !input.is_empty() && input[0].is_ascii_digit() {
        *input = &input[1..];
    }
    let int = &original[..original.len() - input.len()];
    if int.is_empty() {
        return None;
    }
    let int = std::str::from_utf8(int)
        .unwrap() // SAFETY: we know it's ascii digits
        .parse::<u8>()
        .expect("expected integer"); // SAFETY: we know it's ascii digits
    Some(int)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (13, 140));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (6_656, 19_716));
    });
}

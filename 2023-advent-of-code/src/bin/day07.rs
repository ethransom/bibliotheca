#![feature(slice_group_by)]
// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example07.txt");
const INPUT: &str = include_str!("input07.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let mut hands = parse(input);

    // one word per hand! incredible. what could go wrong?! certainly not u16
    // dbg!(std::mem::size_of::<([u8; 5], u16)>()); => 8

    sort_hands(&mut hands);

    (
        hands
            .iter()
            .enumerate()
            .map(|(i, (_cards, bid))| (i + 1) * *bid as usize)
            .sum(),
        0,
    )
}

use Card::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    // IMPORTANT: arranged here in sort order
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    // 11
    Queen,
    // 12
    King,
    // 13
    Ace, // 14
}

use Type::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    // IMPORTANT: arranged here in sort order
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value as char {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!("invalid card value: {}", value),
        }
    }
}

fn sort_hands(hands: &mut Vec<([Card; 5], u16)>) {
    hands.sort_by_key(|(cards, _bid)| {
        (
            hand_type(cards),
            cards[0],
            cards[1],
            cards[2],
            cards[3],
            cards[4],
        )
    });
}

fn hand_type(cards: &[Card; 5]) -> Type {
    let mut cards = cards.clone();
    cards.sort();
    let mut groups: Vec<usize> = cards.group_by(Card::eq).map(|group| group.len()).collect();
    groups.sort_by(|a, b| b.cmp(a)); // descending order

    match groups.as_slice() {
        [5] => FiveOfAKind,
        [4, 1] => FourOfAKind,
        [3, 2] => FullHouse,
        [3, 1, 1] => ThreeOfAKind,
        [2, 2, 1] => TwoPair,
        [2, 1, 1, 1] => OnePair,
        [1, 1, 1, 1, 1] => HighCard,
        _ => panic!("invalid hand: {:?}", cards),
    }
}

fn parse(input: &str) -> Vec<([Card; 5], u16)> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();

            let cards: [u8; 5] = cards.as_bytes().try_into().unwrap();

            (cards.map(Card::from), bid.parse().unwrap())
        })
        .collect()
}

#[test]
fn test_sort_hands_same_type() {
    // KK677 and KTJJT are both two pair. Their first cards both have the
    // same label, but the second card of KK677 is stronger (K vs T), so
    // KTJJT gets rank 2 and KK677 gets rank 3.
    let mut vec = vec![
        (King, King, Six, Seven, Seven),
        (King, Ten, Jack, Jack, Ten),
    ];
    vec.sort();
    assert_eq!(
        vec,
        vec![
            (King, Ten, Jack, Jack, Ten),
            (King, King, Six, Seven, Seven),
        ]
    );

    assert_eq!(
        (King, King, Six, Seven, Seven).cmp(&(King, Ten, Jack, Jack, Ten)),
        std::cmp::Ordering::Greater
    );

    let mut hands = parse("KK677 1\nKTJJT 1");
    sort_hands(&mut hands);
    assert_eq!(hands, parse("KTJJT 1\nKK677 1"));

    // T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first
    // card, so it gets rank 5 and T55J5 gets rank 4.
    let mut hands = parse("T55J5 1\nQQQJA 1");
    sort_hands(&mut hands);
    assert_eq!(hands, parse("T55J5 1\nQQQJA 1"));
}

#[test]
fn test_hand_type() {
    // ty, copilot
    assert_eq!(hand_type(&[Two, Two, Two, Two, Two]), FiveOfAKind);
    assert_eq!(hand_type(&[Two, Two, Two, Two, Three]), FourOfAKind);
    assert_eq!(hand_type(&[Two, Two, Two, Three, Three]), FullHouse);
    assert_eq!(hand_type(&[Two, Two, Two, Three, Four]), ThreeOfAKind);
    assert_eq!(hand_type(&[Two, Two, Three, Three, Four]), TwoPair);
    assert_eq!(hand_type(&[Two, Two, Three, Four, Five]), OnePair);
    assert_eq!(hand_type(&[Two, Three, Four, Five, Six]), HighCard);

    assert_eq!(hand_type(&[Three, Two, Ten, Three, King]), OnePair);
    assert_eq!(hand_type(&[King, King, Six, Seven, Seven]), TwoPair);
    assert_eq!(hand_type(&[King, Ten, Jack, Jack, Ten]), TwoPair);
    assert_eq!(hand_type(&[Ten, Five, Five, Jack, Five]), ThreeOfAKind);
    assert_eq!(hand_type(&[Queen, Queen, Queen, Jack, Ace]), ThreeOfAKind);
}

#[test]
fn test_sort_hands() {
    let mut hands = parse("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n");
    sort_hands(&mut hands);
    assert_eq!(
        hands,
        parse("32T3K 765\nKTJJT 220\nKK677 28\nT55J5 684\nQQQJA 483\n")
    );
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (6440, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (252052080, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }

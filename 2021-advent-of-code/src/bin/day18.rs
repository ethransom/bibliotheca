#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example18.txt");
const INPUT: &str = include_str!("input18.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u32, u32) {
    let sum = input
        .lines()
        .map(Pair::from)
        .reduce(|a, b| a + b)
        .expect("not enough numbers to add")
        .magnitude();

    let pairs: Vec<_> = input.lines().collect();

    let &max = (0..(pairs.len() - 1))
        .flat_map(|left| {
            ((left + 1)..pairs.len())
                .flat_map(|right| {
                    [
                        (Pair::from(String::from(pairs[left]).as_str())
                            + Pair::from(String::from(pairs[right]).as_str()))
                        .magnitude(),
                        (Pair::from(String::from(pairs[right]).as_str())
                            + Pair::from(String::from(pairs[left]).as_str()))
                        .magnitude(),
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .max()
        .expect("not enough numbers to add");

    (sum, max)
}

#[derive(PartialEq)]
struct Pair {
    left: Arm,
    right: Arm,
}

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]", self.left, self.right)
    }
}

#[derive(PartialEq)]
enum Arm {
    Num(u32),
    Branch(Box<Pair>),
}
use Arm::{Branch, Num};

impl std::fmt::Debug for Arm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num(n) => write!(f, "{}", n),
            Branch(b) => write!(f, "{:?}", b),
        }
    }
}

impl From<&str> for Pair {
    fn from(line: &str) -> Pair {
        let mut chars = line.chars().peekable();

        use std::iter::Peekable;
        use std::str::Chars;

        fn recurse(chars: &mut Peekable<Chars>) -> Arm {
            if let Some('[') = chars.peek() {
                assert_eq!(chars.next().expect("unexpected EOF"), '[');

                let left = recurse(chars);

                assert_eq!(chars.next().expect("unexpected EOF"), ',');

                let right = recurse(chars);

                assert_eq!(chars.next().expect("unexpected EOF"), ']');

                Branch(Box::new(Pair { left, right }))
            } else {
                Num(chars
                    .next()
                    .expect("unexpected EOF")
                    .to_digit(10)
                    .expect("unexpected char"))
            }
        }

        match recurse(&mut chars) {
            Branch(pair) => {
                assert_eq!(chars.next(), None);
                *pair
            }
            Num(_) => {
                panic!("root number must be a pair")
            }
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(
        Pair::from("[1,2]"),
        Pair {
            left: Num(1),
            right: Num(2)
        }
    );
    assert_eq!(
        Pair::from("[[1,2],3]"),
        Pair {
            left: Branch(Box::new(Pair {
                left: Num(1),
                right: Num(2)
            })),
            right: Num(3)
        }
    );
    assert_eq!(
        Pair::from("[9,[8,7]]"),
        Pair {
            left: Num(9),
            right: Branch(Box::new(Pair {
                left: Num(8),
                right: Num(7)
            }))
        }
    );
    assert_eq!(
        Pair::from("[[1,9],[8,5]]"),
        Pair {
            left: Branch(Box::new(Pair {
                left: Num(1),
                right: Num(9)
            })),
            right: Branch(Box::new(Pair {
                left: Num(8),
                right: Num(5)
            }))
        }
    );

    // assert no panic
    let _ = Pair::from("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
    let _ = Pair::from("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
    let _ = Pair::from("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
}

impl Arm {
    fn magnitude(&self) -> u32 {
        match &self {
            Num(n) => *n,
            Branch(b) => b.magnitude(),
        }
    }
}

impl Pair {
    fn magnitude(&self) -> u32 {
        3 * &self.left.magnitude() + 2 * &self.right.magnitude()
    }
}

#[test]
fn test_magnitude() {
    assert_eq!(Pair::from("[[1,2],[[3,4],5]]").magnitude(), 143);
    assert_eq!(
        Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
        1384
    );
    assert_eq!(Pair::from("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
    assert_eq!(Pair::from("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
    assert_eq!(
        Pair::from("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),
        1137
    );
    assert_eq!(
        Pair::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
        3488
    );
}

impl std::ops::Add<Pair> for Pair {
    type Output = Pair;

    fn add(self, other: Pair) -> Pair {
        Pair {
            left: Branch(Box::new(self)),
            right: Branch(Box::new(other)),
        }
        .reduce()
    }
}

#[test]
fn test_add() {
    assert_eq!(
        Pair::from("[1,2]") + Pair::from("[[3,4],5]"),
        Pair::from("[[1,2],[[3,4],5]]")
    );

    // no reduction yet, mostly a test of API
    assert_eq!(
        ["[1,1]", "[2,2]", "[3,3]", "[4,4]"]
            .into_iter()
            .map(Pair::from)
            .reduce(|a, b| a + b)
            .unwrap(),
        Pair::from("[[[[1,1],[2,2]],[3,3]],[4,4]]")
    );

    assert_eq!(
        Pair::from("[[[[4,3],4],4],[7,[[8,4],9]]]") + Pair::from("[1,1]"),
        Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );

    // more
    assert_eq!(
        Pair::from("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
            + Pair::from("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
        Pair::from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
    );

    assert_eq!(
        Pair::from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
            + Pair::from("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
        Pair::from("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]")
    );

    assert_eq!(
        Pair::from("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]")
            + Pair::from("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
        Pair::from("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]")
    );

    assert_eq!(
        Pair::from("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]")
            + Pair::from("[7,[5,[[3,8],[1,4]]]]"),
        Pair::from("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]")
    );

    assert_eq!(
        Pair::from("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]")
            + Pair::from("[[2,[2,2]],[8,[8,1]]]"),
        Pair::from("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]")
    );

    assert_eq!(
        Pair::from("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]")
            + Pair::from("[2,9]"),
        Pair::from("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]")
    );

    assert_eq!(
        Pair::from("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]")
            + Pair::from("[1,[[[9,3],9],[[9,0],[0,7]]]]"),
        Pair::from("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]")
    );

    assert_eq!(
        Pair::from("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]")
            + Pair::from("[[[5,[7,4]],7],1]"),
        Pair::from("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]")
    );

    assert_eq!(
        Pair::from("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]")
            + Pair::from("[[[[4,2],2],6],[8,7]]"),
        Pair::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
    );
}

impl Pair {
    fn reduce(self) -> Pair {
        let mut pair = self;

        loop {
            let mut dirty = false;
            pair.explode(&mut dirty, 0);
            if dirty {
                continue;
            }

            if pair.split() {
                continue;
            }

            break;
        }

        pair
    }

    fn explode(&mut self, dirty: &mut bool, depth: usize) -> Option<(u32, u32)> {
        let left_carry = if depth + 1 == 4 {
            let carry = if let Branch(pair) = &mut self.left {
                if let Num(left) = pair.left {
                    if let Num(right) = pair.right {
                        Some((left, right))
                    } else {
                        panic!("tried to explode pair with children on right branch");
                    }
                } else {
                    panic!("tried to explode pair with children on left branch");
                }
            } else {
                None
            };

            if carry.is_some() {
                self.left = Num(0);
                *dirty = true;
            }

            carry
        } else if let Branch(pair) = &mut self.left {
            pair.explode(dirty, depth + 1)
        } else {
            None
        };

        if let Some(left_carry) = left_carry {
            let (left_left_carry, left_right_carry) = left_carry;

            if let Num(right) = self.right {
                if left_right_carry != 0 {
                    self.right = Num(right + left_right_carry);
                    *dirty = true;
                }
                return Some((left_left_carry, 0));
            }
            // right is of Branch
            if left_right_carry != 0 {
                let mut placement = &mut self.right;
                loop {
                    match placement {
                        Num(right) => {
                            *placement = Num(left_right_carry + *right);
                            *dirty = true;
                            break;
                        }
                        Branch(left) => placement = &mut left.left,
                    }
                }
                return Some((left_left_carry, 0));
            }

            return Some(left_carry);
        }

        let right_carry = if depth + 1 == 4 {
            let carry = if let Branch(pair) = &mut self.right {
                if let Num(left) = pair.left {
                    if let Num(right) = pair.right {
                        Some((left, right))
                    } else {
                        panic!("tried to explode pair with children on right branch");
                    }
                } else {
                    panic!("tried to explode pair with children on left branch");
                }
            } else {
                None
            };

            if carry.is_some() {
                self.right = Num(0);
                *dirty = true;
            }

            carry
        } else if let Branch(pair) = &mut self.right {
            pair.explode(dirty, depth + 1)
        } else {
            None
        };

        if let Some(right_carry) = right_carry {
            let (right_left_carry, right_right_carry) = right_carry;

            if let Num(left) = self.left {
                if right_left_carry != 0 {
                    self.left = Num(left + right_left_carry);
                    *dirty = true;
                }
                return Some((0, right_right_carry));
            }
            // right is of Branch
            if right_left_carry != 0 {
                let mut placement = &mut self.left;
                loop {
                    match placement {
                        Num(left) => {
                            *placement = Num(right_left_carry + *left);
                            *dirty = true;
                            break;
                        }
                        Branch(right) => placement = &mut right.right,
                    }
                }
                return Some((0, right_right_carry));
            }

            return Some(right_carry);
        }

        right_carry
    }

    fn split(&mut self) -> bool {
        fn split_variant(arm: &mut Arm) -> bool {
            match arm {
                Branch(b) => b.split(),
                &mut Num(num) => {
                    if num >= 10 {
                        *arm = Branch(Box::new(Pair {
                            left: Num(num / 2),
                            right: Num((num + 1) / 2),
                        }));
                        true
                    } else {
                        false
                    }
                }
            }
        }

        split_variant(&mut self.left) || split_variant(&mut self.right)
    }
}

#[test]
fn test_explode() {
    let mut dirty = false;
    // reduced numbers do not explode
    let mut pair = Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    pair.explode(&mut dirty, 0);
    assert_eq!(pair, Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    assert_eq!(dirty, false);

    // SIMPLE EXPLODES
    // the 9 has no regular number to its left, so it is not added to any regular number
    let mut dirty = false;
    let mut pair = Pair::from("[[[[[9,8],1],2],3],4]");
    pair.explode(&mut dirty, 0);
    assert_eq!(pair, Pair::from("[[[[0,9],2],3],4]"));
    assert_eq!(dirty, true);

    // the 2 has no regular number to its right, and so it is not added to any regular number
    let mut dirty = false;
    let mut pair = Pair::from("[7,[6,[5,[4,[3,2]]]]]");
    pair.explode(&mut dirty, 0);
    assert_eq!(pair, Pair::from("[7,[6,[5,[7,0]]]]"));
    assert_eq!(dirty, true);

    let mut dirty = false;
    let mut pair = Pair::from("[[6,[5,[4,[3,2]]]],1]");
    pair.explode(&mut dirty, 0);
    assert_eq!(pair, Pair::from("[[6,[5,[7,0]]],3]"));
    assert_eq!(dirty, true);

    // the pair [3,2] is unaffected because the pair [7,3] is further to the left; [3,2] would explode on the next action
    let mut dirty = false;
    let mut pair = Pair::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    pair.explode(&mut dirty, 0);
    assert_eq!(pair, Pair::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
    assert_eq!(dirty, true);

    let mut dirty = false;
    let mut pair = Pair::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    pair.explode(&mut dirty, 0);
    assert_eq!(pair, Pair::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    assert_eq!(dirty, true);
}

#[test]
fn test_split() {
    // reduced numbers do not split
    let mut pair = Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    assert_eq!(pair.split(), false);
    assert_eq!(pair, Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

    let mut pair = Pair {
        left: Num(1),
        right: Num(10),
    };
    assert_eq!(pair.split(), true);
    assert_eq!(pair, Pair::from("[1,[5,5]]"));

    let mut pair = Pair {
        left: Num(1),
        right: Num(11),
    };
    assert_eq!(pair.split(), true);
    assert_eq!(pair, Pair::from("[1,[5,6]]"));

    let mut pair = Pair {
        left: Num(1),
        right: Num(12),
    };
    assert_eq!(pair.split(), true);
    assert_eq!(pair, Pair::from("[1,[6,6]]"));
}

#[test]
fn test_reduce_example() {
    // reduced numbers are reduced
    assert_eq!(
        Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").reduce(),
        Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );
    // explode
    assert_eq!(
        Pair::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").reduce(),
        Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );
    // explode
    assert_eq!(
        Pair::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]").reduce(),
        Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );
    // explode
    // NOTE: same as addition example
    assert_eq!(
        Pair::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").reduce(),
        Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (4140, 3993));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (4072, 4483));
    });
}

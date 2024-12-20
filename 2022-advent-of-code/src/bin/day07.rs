#![feature(test)]
#![feature(iter_collect_into)]

extern crate test;

const EXAMPLE: &str = include_str!("example07.txt");
const INPUT: &str = include_str!("input07.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const FILESYSTEM_SIZE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

fn solve(input: &str) -> (usize, usize) {
    let root = parse(input);

    fn size(node: &Node, dir_sizes: &mut Vec<usize>) -> usize {
        match node {
            Node::File { size } => *size,
            Node::Directory { children } => {
                let size = children
                    .iter()
                    .map(|(_name, child)| size(child, dir_sizes))
                    .sum();

                dir_sizes.push(size);

                size
            }
        }
    }

    let mut dir_sizes = vec![];

    let total_size = size(&root, &mut dir_sizes);

    let small_sizes = dir_sizes.iter().filter(|&&size| size <= 100_000).sum();

    dir_sizes.sort_unstable();

    let &to_delete = dir_sizes
        .iter()
        .find(|&&size| FILESYSTEM_SIZE - (total_size - size) >= UPDATE_SIZE)
        .unwrap();

    (small_sizes, to_delete)
}

#[derive(Debug, PartialEq, Eq)]
enum Node {
    File { size: usize },
    Directory { children: Vec<(String, Box<Node>)> },
}

fn parse(input: &str) -> Node {
    let mut commands = input.split("$ ").skip(1).map(|cmd| {
        cmd.split_once('\n')
            .map(|(cmd, output)| {
                let output = output.trim_end();
                if let Some(dir) = cmd.strip_prefix("cd ") {
                    assert!(output.is_empty());
                    Command::Cd {
                        dir: dir.to_string(),
                    }
                } else if cmd.starts_with("ls") {
                    let contents = output
                        .lines()
                        .map(|line| {
                            let (size, name) = line.split_once(' ').unwrap();
                            let size = if size == "dir" {
                                None
                            } else {
                                Some(size.parse().unwrap())
                            };
                            (size, name.to_string())
                        })
                        .collect();
                    Command::Ls { contents }
                } else {
                    panic!("unknown command: {}", cmd);
                }
            })
            .unwrap()
    });

    let mut root = Node::Directory {
        children: Vec::new(),
    };

    assert_eq!(
        commands.next(),
        Some(Command::Cd {
            dir: "/".to_owned()
        })
    );

    fn instantiate(cmds: &mut impl Iterator<Item = Command>, mut node: &mut Node) {
        while let Some(cmd) = cmds.next() {
            match cmd {
                Command::Cd { dir } => {
                    if dir == ".." {
                        return;
                    }
                    let node = match &mut node {
                        Node::Directory { children } => children
                            .iter_mut()
                            .find(|(name, _)| name == &dir)
                            .unwrap_or_else(|| panic!("no child named {}", dir)),
                        Node::File { .. } => panic!("cd into file"),
                    };
                    instantiate(cmds, &mut node.1);
                }
                Command::Ls { contents } => {
                    match &mut node {
                        Node::Directory { children } => {
                            assert!(children.is_empty());
                            contents
                                .into_iter()
                                .map(|(size, name)| {
                                    let node = match size {
                                        Some(size) => Node::File { size },
                                        None => Node::Directory {
                                            children: Vec::new(),
                                        },
                                    };
                                    (name, Box::new(node))
                                })
                                .collect_into(children);
                        }
                        Node::File { .. } => panic!("ls into file"),
                    };
                }
            }
        }
    }

    instantiate(&mut commands, &mut root);

    root
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Cd {
        dir: String,
    },
    Ls {
        contents: Vec<(Option<usize>, String)>,
    },
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (95_437, 24_933_642));
}

#[bench]
fn bench_solve_00_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (1_423_358, 545_729));
    });
}

#[bench]
fn bench_solve_01_less_indirection(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let root = parse(input);

        fn size(node: &Node, dir_sizes: &mut Vec<usize>) -> usize {
            match node {
                Node::File { size } => *size,
                Node::Directory { children } => {
                    let size = children
                        .iter()
                        .map(|(_name, child)| size(child, dir_sizes))
                        .sum();

                    dir_sizes.push(size);

                    size
                }
            }
        }

        let mut dir_sizes = vec![];

        let total_size = size(&root, &mut dir_sizes);

        let small_sizes = dir_sizes.iter().filter(|&&size| size <= 100_000).sum();

        dir_sizes.sort_unstable();

        let &to_delete = dir_sizes
            .iter()
            .find(|&&size| FILESYSTEM_SIZE - (total_size - size) >= UPDATE_SIZE)
            .unwrap();

        (small_sizes, to_delete)
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Node {
        File { size: usize },
        Directory { children: Vec<(String, Node)> },
    }

    fn parse(input: &str) -> Node {
        let mut commands = input.split("$ ").skip(1).map(|cmd| {
            cmd.split_once('\n')
                .map(|(cmd, output)| {
                    let output = output.trim_end();
                    if let Some(dir) = cmd.strip_prefix("cd ") {
                        assert!(output.is_empty());
                        Command::Cd {
                            dir: dir.to_string(),
                        }
                    } else if cmd.starts_with("ls") {
                        let contents = output
                            .lines()
                            .map(|line| {
                                let (size, name) = line.split_once(' ').unwrap();
                                let size = if size == "dir" {
                                    None
                                } else {
                                    Some(size.parse().unwrap())
                                };
                                (size, name.to_string())
                            })
                            .collect();
                        Command::Ls { contents }
                    } else {
                        panic!("unknown command: {}", cmd);
                    }
                })
                .unwrap()
        });

        let mut root = Node::Directory {
            children: Vec::new(),
        };

        assert_eq!(
            commands.next(),
            Some(Command::Cd {
                dir: "/".to_owned()
            })
        );

        fn instantiate(cmds: &mut impl Iterator<Item = Command>, mut node: &mut Node) {
            while let Some(cmd) = cmds.next() {
                match cmd {
                    Command::Cd { dir } => {
                        if dir == ".." {
                            return;
                        }
                        let node = match &mut node {
                            Node::Directory { children } => children
                                .iter_mut()
                                .find(|(name, _)| name == &dir)
                                .unwrap_or_else(|| panic!("no child named {}", dir)),
                            Node::File { .. } => panic!("cd into file"),
                        };
                        instantiate(cmds, &mut node.1);
                    }
                    Command::Ls { contents } => {
                        match &mut node {
                            Node::Directory { children } => {
                                assert!(children.is_empty());
                                contents
                                    .into_iter()
                                    .map(|(size, name)| {
                                        let node = match size {
                                            Some(size) => Node::File { size },
                                            None => Node::Directory {
                                                children: Vec::new(),
                                            },
                                        };
                                        (name, node)
                                    })
                                    .collect_into(children);
                            }
                            Node::File { .. } => panic!("ls into file"),
                        };
                    }
                }
            }
        }

        instantiate(&mut commands, &mut root);

        root
    }

    assert_eq!(solve(EXAMPLE), (95_437, 24_933_642));

    b.iter(|| {
        assert_eq!(solve(INPUT), (1_423_358, 545_729));
    });
}

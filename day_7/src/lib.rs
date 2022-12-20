#![feature(iter_intersperse)]
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, alphanumeric1, line_ending, not_line_ending},
    },
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
enum Command<'a> {
    Ls(Vec<FileType<'a>>),
    CdUp,
    CdRoot,
    Cd(&'a str),
}

fn parse_file(input: &str) -> IResult<&str, File> {
    map(
        separated_pair(character::complete::u64, tag(" "), not_line_ending),
        |(size, name)| File { name, size },
    )(input)
}

fn parse_dir(input: &str) -> IResult<&str, &str> {
    preceded(tag("dir "), alphanumeric1)(input)
}

fn parse_file_type(input: &str) -> IResult<&str, FileType> {
    alt((
        parse_file.map(|f| FileType::File(f)),
        parse_dir.map(|n| FileType::Dir(n)),
    ))(input)
}

fn parse_dir_content(input: &str) -> IResult<&str, Vec<FileType>> {
    separated_list0(line_ending, parse_file_type)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, command) = preceded(
        tag("$ "),
        alt((
            tag("ls"),
            tag("cd .."),
            tag("cd /"),
            preceded(tag("cd "), alpha1),
        )),
    )(input)?;
    match command {
        "ls" => map(preceded(line_ending, parse_dir_content), |d| Command::Ls(d))(input),
        "cd .." => Ok((input, Command::CdUp)),
        "cd /" => Ok((input, Command::CdRoot)),
        name => Ok((input, Command::Cd(name))),
    }
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(line_ending, parse_command)(input)
}

pub fn process1(input: &str) -> u64 {
    let (_, commands) = commands(input).unwrap();
    println!("{commands:?}");
    let (map, _) = commands.iter().fold(
        (HashMap::new(), Vec::<String>::new()),
        |(mut state, mut stack), c| {
            match c {
                Command::Ls(a) => {
                    let dir_sum = a
                        .iter()
                        .map(|ft| match ft {
                            FileType::Dir(_) => 0,
                            FileType::File(f) => f.size,
                        })
                        .sum();
                    state
                        .entry(
                            stack
                                .iter()
                                .cloned()
                                .intersperse("|".to_string())
                                .collect::<String>(),
                        )
                        .and_modify(|size| *size += dir_sum)
                        .or_insert(dir_sum);
                }
                Command::CdUp => {
                    stack.pop();
                }
                Command::CdRoot => {
                    stack.push(String::from("/"));
                    state.insert(String::from("/"), 0);
                }
                Command::Cd(name) => {
                    stack.push(name.to_string());
                    state.insert(
                        stack
                            .iter()
                            .cloned()
                            .intersperse("|".to_string())
                            .collect::<String>(),
                        0,
                    );
                }
            }
            (state, stack)
        },
    );
    println!("MAP {map:?}");
    let a = map.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.entry(&k[..]).and_modify(|sum| *sum += v).or_insert(*v);
        let mut key = &k[..];
        while let Some((rest, _)) = key.rsplit_once("|") {
            acc.entry(rest).and_modify(|sum| *sum += v).or_insert(*v);
            key = rest;
        }
        acc
    });
    println!("MAP3 {a:?}");
    a.values().filter(|&&size| size <= 100000).sum()
}

pub fn process2(input: &str) -> u64 {
    let (_, commands) = commands(input).unwrap();
    let (map, _) = commands.iter().fold(
        (HashMap::new(), Vec::<String>::new()),
        |(mut state, mut stack), c| {
            match c {
                Command::Ls(a) => {
                    let dir_sum = a
                        .iter()
                        .map(|ft| match ft {
                            FileType::Dir(_) => 0,
                            FileType::File(f) => f.size,
                        })
                        .sum();
                    state
                        .entry(
                            stack
                                .iter()
                                .cloned()
                                .intersperse("|".to_string())
                                .collect::<String>(),
                        )
                        .and_modify(|size| *size += dir_sum)
                        .or_insert(dir_sum);
                }
                Command::CdUp => {
                    stack.pop();
                }
                Command::CdRoot => {
                    stack.push(String::from("/"));
                    state.insert(String::from("/"), 0);
                }
                Command::Cd(name) => {
                    stack.push(name.to_string());
                    state.insert(
                        stack
                            .iter()
                            .cloned()
                            .intersperse("|".to_string())
                            .collect::<String>(),
                        0,
                    );
                }
            }
            (state, stack)
        },
    );
    let a = map.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.entry(&k[..]).and_modify(|sum| *sum += v).or_insert(*v);
        let mut key = &k[..];
        while let Some((rest, _)) = key.rsplit_once("|") {
            acc.entry(rest).and_modify(|sum| *sum += v).or_insert(*v);
            key = rest;
        }
        acc
    });
    // a.values()
    // // .filter(|&&size| size <= 100000)
    // .sum();
    let diff = 30000000u64.abs_diff(70000000 - a["/"]);
    *a.values().filter(|&&size| size > diff).min().unwrap()
}

// #[derive(Debug)]
// struct Dir<'a> {
//     name: &'a str,
//     children: Vec<Rc<RefCell<FileType>>>,
//     size: u64,
// }

#[derive(Debug)]
enum FileType<'a> {
    Dir(&'a str),
    File(File<'a>),
}

// impl Dir {
//     fn calc_size(&mut self) -> u64 {
//         self.children
//             .iter_mut()
//             .map(|c| match c {
//                 FileType::Dir(d) => d.calc_size(),
//                 FileType::File(f) => f.size,
//             })
//             .sum()
//     }
// }

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        let result = process1(&input);
        println!("Result part one: {result}")
    }

    #[test]
    fn process_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        let result = process2(&input);
        println!("Result part two: {result}")
    }

    #[test]
    fn process_one_test() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let result = process1(input);
        assert_eq!(result, 95437)
    }

    #[test]
    fn process_two_test() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let result = process2(input);
        assert_eq!(result, 24933642)
    }
}

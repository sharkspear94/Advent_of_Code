use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::{self, anychar, line_ending, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Eval<'a> {
    Num(f64),
    Add(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn parse_op(input: &str) -> IResult<&str, Eval> {
    map(
        tuple((
            terminated(alpha1, multispace1),
            anychar,
            preceded(multispace1, alpha1),
        )),
        |(a, op, b)| match op {
            '+' => Eval::Add(a, b),
            '-' => Eval::Sub(a, b),
            '*' => Eval::Mul(a, b),
            '/' => Eval::Div(a, b),
            r => panic!("operation {r} not suppported"),
        },
    )(input)
}

fn parse_eval(input: &str) -> IResult<&str, (&str, Eval)> {
    separated_pair(
        alpha1,
        tag(": "),
        alt((map(complete::i64, |num| Eval::Num(num as f64)), parse_op)),
    )(input)
}

fn parse_puzzle(input: &str) -> IResult<&str, HashMap<&str, Eval>> {
    separated_list1(alt((line_ending,)), parse_eval)(input).map(|(i, v)| (i, HashMap::from_iter(v)))
}

fn solve(key: &str, puzzle: &HashMap<&str, Eval>) -> f64 {
    match puzzle[key] {
        Eval::Num(n) => n,
        Eval::Add(a, b) => solve(a, puzzle) + solve(b, puzzle),
        Eval::Mul(a, b) => solve(a, puzzle) * solve(b, puzzle),
        Eval::Sub(a, b) => solve(a, puzzle) - solve(b, puzzle),
        Eval::Div(a, b) => solve(a, puzzle) / solve(b, puzzle),
    }
}

#[derive(Debug)]
enum Op {
    Mul(f64),
    Add(f64),
    Sub(f64, Pos),
    Div(f64, Pos),
}

#[derive(Debug)]
enum Pos {
    Left,
    Right,
}

fn solve_2(key: &str, puzzle: &HashMap<&str, Eval>, path: &mut Vec<Op>) -> bool {
    if key == "humn" {
        return true;
    }
    match puzzle[key] {
        Eval::Num(_) => false,
        Eval::Add(a, b) => {
            let mut res = false;
            if solve_2(b, puzzle, path) {
                path.push(Op::Add(solve(a, puzzle)));
                res = true
            } else if solve_2(a, puzzle, path) {
                path.push(Op::Add(solve(b, puzzle)));
                res = true
            }
            res
        }
        Eval::Mul(a, b) => {
            let mut res = false;
            if solve_2(b, puzzle, path) {
                path.push(Op::Mul(solve(a, puzzle)));
                res = true
            } else if solve_2(a, puzzle, path) {
                path.push(Op::Mul(solve(b, puzzle)));
                res = true
            }
            res
        }
        Eval::Sub(a, b) => {
            let mut res = false;
            if solve_2(b, puzzle, path) {
                path.push(Op::Sub(solve(a, puzzle), Pos::Left));
                res = true
            } else if solve_2(a, puzzle, path) {
                path.push(Op::Sub(solve(b, puzzle), Pos::Right));
                res = true
            }
            res
        }
        Eval::Div(a, b) => {
            let mut res = false;
            if solve_2(b, puzzle, path) {
                path.push(Op::Div(solve(a, puzzle), Pos::Left));
                res = true
            } else if solve_2(a, puzzle, path) {
                path.push(Op::Div(solve(b, puzzle), Pos::Right));
                res = true
            }
            res
        }
    }
}
pub fn process1(input: &str) -> i64 {
    let (_, puzzle) = parse_puzzle(input).unwrap();
    solve("root", &puzzle) as i64
}

pub fn process2(input: &str) -> i64 {
    let (_, puzzle) = parse_puzzle(input).unwrap();
    let mut path = vec![];
    solve_2("root", &puzzle, &mut path);

    // println!("{:?}", path);
    let equal = match path.pop().unwrap() {
        Op::Mul(num) => num,
        Op::Add(num) => num,
        Op::Sub(num, _) => num,
        Op::Div(num, _) => num,
    };

    let res = path.iter().rev().fold(equal, |a, op| match op {
        Op::Mul(num) => a / num,
        Op::Add(num) => a - num,
        Op::Sub(num, Pos::Left) => -(a - num),
        Op::Sub(num, Pos::Right) => a + num,
        Op::Div(num, _) => a * num,
    });
    println!("{res}");
    res as i64
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
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        let result = process1(input);
        assert_eq!(result, 152)
    }

    #[test]
    fn process_two_test() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        let result = process2(input);
        assert_eq!(result, 301)
    }
}

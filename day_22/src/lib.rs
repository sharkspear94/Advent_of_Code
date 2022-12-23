#![feature(iter_advance_by)]

use std::ops::Range;

use nom::{
    branch::alt,
    character::{
        self,
        complete::{line_ending, satisfy, space0},
    },
    combinator::{iterator, map},
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult,
};

fn maze_line(input: &str) -> IResult<&str, Vec<Tile>> {
    let (input, mut line) = map(space0, |s: &str| vec![Tile::Empty; s.len()])(input)?;
    let mut it = iterator(input, satisfy(|c| c == '#' || c == '.'));
    line.extend(it.map(|c| match c {
        '#' => Tile::Wall,
        _ => Tile::Tile,
    }));
    let (i, _) = it.finish()?;
    Ok((i, line))
}

fn maze(input: &str) -> IResult<&str, Vec<(Range<usize>, Vec<Tile>)>> {
    fold_many1(
        terminated(maze_line, line_ending),
        Vec::new,
        |mut acc, el| {
            let first = el
                .iter()
                .position(|t| !matches!(t, Tile::Empty))
                .unwrap_or_default();
            if el.len() > 0 {
                acc.push(((first..el.len()), el));
            }
            acc
        },
    )(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Walk>> {
    many1(alt((
        map(character::complete::u32, |d| Walk::Step(d as usize)),
        map(satisfy(|c| c == 'L' || c == 'R'), |c| match c {
            'L' => Walk::Turn(Direction::Left),
            _ => Walk::Turn(Direction::Right),
        }),
    )))(input)
}

fn form_steps(ins: Vec<Walk>) -> Vec<Step> {
    let mut steps = ins[1..]
        .chunks(2)
        .scan(Direction::Right, |current_dir, pair| {
            let steps = match pair[1] {
                Walk::Step(s) => s,
                Walk::Turn(_) => panic!(""),
            };
            match (&current_dir, &pair[0]) {
                (Direction::Right, Walk::Turn(Direction::Left)) => {
                    *current_dir = Direction::Up;
                    Some(Step {
                        steps,
                        dir: current_dir.clone(),
                    })
                }
                (Direction::Left, Walk::Turn(Direction::Left)) => {
                    *current_dir = Direction::Down;
                    Some(Step {
                        steps,
                        dir: current_dir.clone(),
                    })
                }
                (Direction::Down, Walk::Turn(Direction::Left)) => {
                    *current_dir = Direction::Right;
                    Some(Step {
                        steps,
                        dir: current_dir.clone(),
                    })
                }
                (Direction::Up, Walk::Turn(Direction::Left)) => {
                    *current_dir = Direction::Left;
                    Some(Step {
                        steps,
                        dir: current_dir.clone(),
                    })
                }
                (Direction::Right, _) => {
                    *current_dir = Direction::Down;
                    Some(Step {
                        steps,
                        dir: current_dir.clone(),
                    })
                }
                (Direction::Left, _) => {
                    *current_dir = Direction::Up;
                    Some(Step {
                        steps,
                        dir: current_dir.clone(),
                    })
                }
                (Direction::Down, _) => {
                    *current_dir = Direction::Left;
                    Some(Step {
                        steps,
                        dir: current_dir.clone(),
                    })
                }
                (Direction::Up, _) => {
                    *current_dir = Direction::Right;
                    Some(Step {
                        steps,
                        dir: current_dir.clone(),
                    })
                }
            }
        })
        .collect::<Vec<_>>();

    steps.insert(
        0,
        Step {
            steps: match ins[0] {
                Walk::Step(s) => s,
                Walk::Turn(_) => panic!(),
            },
            dir: Direction::Right,
        },
    );
    steps
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Tile,
    Wall,
    Empty,
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}
#[derive(Debug)]
enum Walk {
    Step(usize),
    Turn(Direction),
}
#[derive(Debug)]
struct Step {
    steps: usize,
    dir: Direction,
}

fn walk_step(
    maze: &Vec<(std::ops::Range<usize>, Vec<Tile>)>,
    step: &Step,
    (x, y): (usize, usize),
) -> (usize, usize) {
    match step.dir {
        Direction::Right => {
            let a = maze[y]
                .0
                .clone()
                .cycle()
                .skip(x.saturating_sub(maze[y].0.start))
                .take(step.steps + 1)
                .take_while(|&i| maze[y].1[i] == Tile::Tile)
                .last()
                .unwrap();
            (a, y)
        }
        Direction::Left => {
            let a = maze[y]
                .0
                .clone()
                .rev()
                .cycle()
                .skip((maze[y].0.end - 1).saturating_sub(x))
                .take(step.steps + 1)
                .take_while(|&i| maze[y].1[i] == Tile::Tile)
                .last()
                .unwrap();
            (a, y)
        }
        Direction::Down => {
            let min_height = maze
                .iter()
                .position(|(range, _)| range.contains(&x))
                .unwrap();
            let max_height = maze.len()
                - maze
                    .iter()
                    .rev()
                    .position(|(range, _)| range.contains(&x))
                    .unwrap();
            let a = (min_height..max_height)
                .cycle()
                .skip(y.saturating_sub(min_height))
                .take(step.steps + 1)
                .take_while(|&i| maze[i].1[x] == Tile::Tile)
                .last()
                .unwrap();
            (x, a)
        }
        Direction::Up => {
            let min_height = maze
                .iter()
                .position(|(range, _)| range.contains(&x))
                .unwrap();
            let max_height = maze.len()
                - maze
                    .iter()
                    .rev()
                    .position(|(range, _)| range.contains(&x))
                    .unwrap();
            let a = (min_height..max_height)
                .rev()
                .cycle()
                .skip((max_height - 1).saturating_sub(y))
                .inspect(|&i| println!("y: {i:?}"))
                .take(step.steps + 1)
                .take_while(|&i| maze[i].1[x] == Tile::Tile)
                .last()
                .unwrap();
            // println!("up: x:{x},y{a}");
                (x, a)
        }
    }
}

fn walk_maze(
    maze: &Vec<(std::ops::Range<usize>, Vec<Tile>)>,
    steps: &Vec<Step>,
) -> (usize, usize, Direction) {
    let mut current_pos = (maze[0].0.start, 0);
    for step in steps {
        current_pos = walk_step(maze, step, current_pos);
    }
    (current_pos.0, current_pos.1, steps.last().unwrap().dir)
}

pub fn process1(input: &str) -> usize {
    let (i, mut maze) = maze(input).unwrap();
    let max_len = maze.iter().map(|l| l.1.len()).max().unwrap();
    maze.iter_mut()
        .filter(|(_, row)| row.len() < max_len)
        .for_each(|(_, row)| {
            let len = row.len();
            row.extend((len..max_len).map(|_| Tile::Empty));
        });
    let (_, ins) = instructions(i).unwrap();
    let steps = form_steps(ins);
    let (col, row, dir) = walk_maze(&maze, &steps);
    1000 * (row + 1)
        + 4 * (col + 1)
        + match dir {
            Direction::Right => 0,
            Direction::Left => 2,
            Direction::Down => 1,
            Direction::Up => 3,
        }
}

pub fn process2(input: &str) -> i64 {
    todo!()
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
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let result = process1(input);
        assert_eq!(result, 6032)
    }

    #[test]
    fn process_two_test() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let result = process2(input);
        assert_eq!(result, 1623178306)
    }
}

#![feature(iter_advance_by)]
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, EnumIter, Clone, Copy)]
enum Dirs {
    North,
    South,
    West,
    East,
}

fn check_direction(
    (x, y): (i32, i32),
    &[north, south, west, east]: &[bool; 4],
    dir: Dirs,
) -> Option<((i32, i32), (i32, i32))> {
    match dir {
        Dirs::North if north => Some(((x, y), (x, y - 1))),
        Dirs::South if south => Some(((x, y), (x, y + 1))),
        Dirs::West if west => Some(((x, y), (x - 1, y))),
        Dirs::East if east => Some(((x, y), (x + 1, y))),
        _ => None,
    }
}

fn move_to<I: Iterator<Item = Dirs> + Clone>(
    (x, y): (i32, i32),
    set: &HashSet<(i32, i32)>,
    dir_iter: I,
) -> Option<((i32, i32), (i32, i32))> {
    let north = !set.contains(&(x - 1, y - 1))
        && !set.contains(&(x, y - 1))
        && !set.contains(&(x + 1, y - 1));
    let south = !set.contains(&(x - 1, y + 1))
        && !set.contains(&(x, y + 1))
        && !set.contains(&(x + 1, y + 1));
    let west = !set.contains(&(x - 1, y + 1))
        && !set.contains(&(x - 1, y))
        && !set.contains(&(x - 1, y - 1));
    let east = !set.contains(&(x + 1, y + 1))
        && !set.contains(&(x + 1, y))
        && !set.contains(&(x + 1, y - 1));
    if north && south && west && east {
        None
    } else {
        dir_iter
            .take(4)
            .find_map(|dir| check_direction((x, y), &[north, south, west, east], dir))
    }
}

pub fn process1(input: &str) -> i32 {
    let mut set = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect::<HashSet<_>>();
    let mut dir_iter = Dirs::iter().cycle();
    for _ in 1..=10 {
        let changes = set
            .iter()
            .filter_map(|&coord| move_to(coord, &set, dir_iter.clone()))
            .fold(HashMap::new(), |mut map, (old, new)| {
                map.entry(new)
                    .and_modify(|(_, e)| {
                        *e += 1;
                    })
                    .or_insert((old, 1));
                map
            })
            .into_iter()
            .filter(|&(_, (_, count))| count == 1)
            .map(|(new, (old, _))| (old, new))
            .collect::<Vec<_>>();
        // println!("canges: {changes:?}");
        changes.into_iter().for_each(|(old, new)| {
            if !set.remove(&old) {
                println!("and not present removed");
            }
            if !set.insert(new) {
                println!("insert override at :{:?} and deleted: {old:?}", new);
            }
        });
        dir_iter.advance_by(1).unwrap();
    }
    let (min_y, max_y) = set
        .iter()
        .map(|coord| coord.1)
        .minmax_by(|a, b| a.cmp(&b))
        .into_option()
        .unwrap();
    let (min_x, max_x) = set
        .iter()
        .map(|coord| coord.0)
        .minmax_by(|a, b| a.cmp(&b))
        .into_option()
        .unwrap();
    println!("min x: {min_x}, max x: {max_x}");
    println!("y: {}", max_y.abs_diff(min_y));
    println!("x: {}", max_x.abs_diff(min_x));
    (max_y + 1 - min_y) * (max_x + 1 - min_x) - set.len() as i32
}

pub fn process2(input: &str) -> i32 {
    let mut set = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect::<HashSet<_>>();
    // println!("set len: {}", set.len());
    let mut dir_iter = Dirs::iter().cycle();
    // println!("set before: {set:?}");
    for round in 1.. {
        let changes = set
            .iter()
            .filter_map(|&coord| move_to(coord, &set, dir_iter.clone()))
            .fold(HashMap::new(), |mut map, (old, new)| {
                map.entry(new)
                    .and_modify(|(_, e)| {
                        *e += 1;
                    })
                    .or_insert((old, 1));
                map
            })
            .into_iter()
            .filter(|&(_, (_, count))| count == 1)
            .map(|(new, (old, _))| (old, new))
            // .inspect(|coords| println!("{coords:?}"))
            .collect::<Vec<_>>();
        // println!("canges: {changes:?}");
        if changes.len() == 0 {
            return round;
        }
        changes.into_iter().for_each(|(old, new)| {
            if !set.remove(&old) {
                println!("and not present removed");
            }
            if !set.insert(new) {
                println!("insert override at :{:?} and deleted: {old:?}", new);
            }
        });
        // println!("set after round {round}: {set:?}");
        dir_iter.advance_by(1).unwrap();
    }
    panic!()
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
        let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        let result = process1(input);
        assert_eq!(result, 110)
    }

    #[test]
    fn process_two_test() {
        let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        let result = process2(input);
        assert_eq!(result, 20)
    }
}

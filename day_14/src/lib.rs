#![feature(array_windows)]

use itertools::{Itertools, MinMaxResult};
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{line_ending, newline},
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_trace(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(character::complete::u32, tag(","), character::complete::u32)(input)
}

fn parse_path(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(tag(" -> "), parse_trace)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    separated_list1(line_ending, parse_path)(input)
}

fn build_grid(paths: Vec<Vec<(u32, u32)>>) -> (Vec<Vec<char>>, usize) {
    let (min, max) = paths
        .iter()
        .flatten()
        .map(|(width, _)| width)
        .minmax()
        .into_option()
        .unwrap();
    let (min, max) = (*min, *max);
    let width = (max - min) as usize + 1;
    let (_, max_height) = paths.iter().flatten().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let mut grid = vec![vec!['.'; width]; *max_height as usize + 1];

    for path in paths {
        for [(x1, y1), (x2, y2)] in path.array_windows() {
            let x1 = x1 - min;
            let x2 = x2 - min;
            if x1 == x2 {
                let len = y1.abs_diff(*y2) as usize + 1;
                let start = *std::cmp::min(y1, y2) as usize;
                grid.iter_mut()
                    .skip(start)
                    .take(len)
                    .for_each(|row| row[x1 as usize] = '#')
            } else {
                let len = x1.abs_diff(x2) as usize + 1;
                let start = std::cmp::min(x1, x2) as usize;
                grid[*y1 as usize]
                    .iter_mut()
                    .skip(start)
                    .take(len)
                    .for_each(|c| *c = '#')
            }
        }
    }
    (grid, 500 - min as usize)
}

enum Moves {
    NextMove(usize, usize),
    Resting(usize, usize),
    Overflow,
}

fn move_sandcorn(grid: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Moves {
    let Some(row) = grid.get(y + 1).filter(|_| x>0).filter(|row| x<(row.len()-1) ) else {
            return Moves::Overflow;
    };

    if let Some(next_move) = Some(row[x])
        .filter(|&c| c == '.')
        .map(|_| Moves::NextMove(x, y + 1))
        .or(Some(row[x - 1])
            .filter(|&c| c == '.')
            .map(|_| Moves::NextMove(x - 1, y + 1)))
        .or(Some(row[x + 1])
            .filter(|&c| c == '.')
            .map(|_| Moves::NextMove(x + 1, y + 1)))
    {
        next_move
    } else {
        Moves::Resting(x, y)
    }
}

fn place_sandcorn(grid: &mut Vec<Vec<char>>, start: usize) -> bool {
    let mut current = (start, 0);
    loop {
        match move_sandcorn(&grid, current) {
            Moves::NextMove(x, y) => current = (x, y),
            Moves::Resting(x, y) => {
                if x == start && y == 0 {
                    grid[y][x] = 'o';
                    return false;
                }
                grid[y][x] = 'o';
                return true;
            }
            Moves::Overflow => return false,
        }
    }
}

fn populate_grid(grid: &mut Vec<Vec<char>>, start: usize) {
    while place_sandcorn(grid, start) {}
}

pub fn proccess_one(input: &str) -> usize {
    let paths = parse(input).unwrap().1;
    let (mut grid, start) = build_grid(paths);
    populate_grid(&mut grid, start);
    grid.iter().flatten().filter(|&&c| c == 'o').count()
}

// --------------------------------------------------------

fn build_grid2(paths: Vec<Vec<(u32, u32)>>) -> (Vec<Vec<char>>, usize) {
    let (min, max) = paths
        .iter()
        .flatten()
        .map(|(width, _)| width)
        .minmax()
        .into_option()
        .unwrap();
    let (min, max) = (*min, *max);
    let width = (max - min) as usize + 3;
    let (_, max_height) = paths.iter().flatten().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let mut grid = vec![vec!['.'; width]; *max_height as usize + 1];

    for path in paths {
        for [(x1, y1), (x2, y2)] in path.array_windows() {
            let x1 = x1 - min + 1;
            let x2 = x2 - min + 1;
            if x1 == x2 {
                let len = y1.abs_diff(*y2) as usize + 1;
                let start = *std::cmp::min(y1, y2) as usize;
                grid.iter_mut()
                    .skip(start)
                    .take(len)
                    .for_each(|row| row[x1 as usize] = '#')
            } else {
                let len = x1.abs_diff(x2) as usize + 1;
                let start = std::cmp::min(x1, x2) as usize;
                grid[*y1 as usize]
                    .iter_mut()
                    .skip(start)
                    .take(len)
                    .for_each(|c| *c = '#')
            }
        }
    }
    (grid, 500 - min as usize + 1)
}

fn move_sandcorn2(grid: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Moves {
    if let Some(next_move) = grid
        .get(y + 1)
        .filter(|row| row[x] == '.')
        .map(|_| Moves::NextMove(x, y + 1))
        .or_else(|| {
            grid.get(y + 1)
                .and_then(|row| x.checked_sub(1).map(|x| row[x]).filter(|&c| c == '.'))
                .map(|_| Moves::NextMove(x - 1, y + 1))
        })
        .or_else(|| {
            grid.get(y + 1)
                .and_then(|row| row.get(x + 1))
                .filter(|&&c| c == '.')
                .map(|_| Moves::NextMove(x + 1, y + 1))
        })
    {
        next_move
    } else {
        Moves::Resting(x, y)
    }
}

fn place_sandcorn2(grid: &mut Vec<Vec<char>>, start: usize) -> bool {
    let mut current = (start, 0);
    loop {
        match move_sandcorn2(&grid, current) {
            Moves::NextMove(x, y) => current = (x, y),
            Moves::Resting(x, y) => {
                if x == start && y == 0 {
                    grid[y][x] = 'o';
                    return false;
                }
                grid[y][x] = 'o';
                return true;
            }
            Moves::Overflow => return false,
        }
    }
}

fn populate_grid2(grid: &mut Vec<Vec<char>>, start: usize) {
    while place_sandcorn2(grid, start) {}
}

fn missing_triangle(len: usize) -> usize {
    (len * (len + 1)) / 2
}

pub fn proccess_two(input: &str) -> usize {
    let paths = parse(input).unwrap().1;
    let (mut grid, start) = build_grid2(paths);
    let width = grid[0].len();
    grid.push(vec!['.'; width]);
    grid.push(vec!['#'; width]);
    populate_grid2(&mut grid, start);

    let (rest_left_pos, _) = grid
        .iter()
        .find_position(|row| row.first().unwrap() == &'o')
        .unwrap();
    let (rest_right_pos, _) = grid
        .iter()
        .find_position(|row| row.last().unwrap() == &'o')
        .unwrap();

    let rest_left = missing_triangle(grid.len() - rest_left_pos - 2);
    let rest_right = missing_triangle(grid.len() - rest_right_pos - 2);
    grid.iter().flatten().filter(|&&c| c == 'o').count() + rest_left + rest_right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part one: {}", proccess_one(&input));
    }

    #[test]
    fn part_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part two: {}", proccess_two(&input));
    }

    #[test]
    fn test_part_one() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(proccess_one(input), 24);
    }

    #[test]
    fn test_part_two() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(proccess_two(input), 93);
    }
}

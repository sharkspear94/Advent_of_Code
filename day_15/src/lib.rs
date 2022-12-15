use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::{Range, RangeInclusive},
};

use nom::{
    bytes::complete::tag,
    character::{self, complete::line_ending},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
#[derive(Debug, Default, PartialEq, PartialOrd, Hash, Eq)]
struct Sensor(i32, i32);
#[derive(Debug, Default, PartialEq, PartialOrd, Hash, Eq)]
struct Beacon(i32, i32);

impl Sensor {
    fn line_intersections(&self, beacon: &Beacon, line: i32) -> Option<RangeInclusive<i32>> {
        let dist = (self.0.abs_diff(beacon.0) + self.1.abs_diff(beacon.1)) as i32;
        if (line >= self.1 && line <= self.1 + dist) || (line <= self.1 && line >= self.1 - dist) {
            let y_diff = self.1.abs_diff(line) as i32;
            Some((self.0 - (dist - y_diff))..=(self.0 + (dist - y_diff)))
        } else {
            None
        }
    }

    fn line_intersections_2(
        &self,
        beacon: &Beacon,
        line: i32,
        search_space: i32,
    ) -> Option<RangeInclusive<i32>> {
        let dist = (self.0.abs_diff(beacon.0) + self.1.abs_diff(beacon.1)) as i32;
        if (line >= self.1 && line <= self.1 + dist) || (line <= self.1 && line >= self.1 - dist) {
            let y_diff = self.1.abs_diff(line) as i32;
            Some(max(self.0 - (dist - y_diff), 0)..=min(self.0 + (dist - y_diff), search_space))
        } else {
            None
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, sensor) = delimited(
        tag("Sensor at x="),
        separated_pair(
            character::complete::i32,
            tag(", y="),
            character::complete::i32,
        )
        .map(|a| Sensor(a.0, a.1)),
        tag(": closest beacon is at x="),
    )(input)?;
    let (input, (x, y)) = separated_pair(
        character::complete::i32,
        tag(", y="),
        character::complete::i32,
    )(input)?;
    Ok((input, (sensor, Beacon(x, y))))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Sensor, Beacon)>> {
    nom::multi::separated_list1(line_ending, parse_line)(input)
}

fn merge_ranges(ranges: &mut [RangeInclusive<i32>], r: &RangeInclusive<i32>) -> bool {
    for range in ranges {
        if (range.contains(&r.start()) || range.contains(&r.end()))
            || (r.contains(&range.start()) || r.contains(&range.end()))
        {
            *range = min(*r.start(), *range.start())..=max(*r.end(), *range.end());
            return true;
        }
    }
    false
}

fn fold_rages(mut ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    ranges.sort_by(|a, b| a.start().cmp(&b.start()));
    ranges.into_iter().fold(vec![], |mut acc, r| {
        if !merge_ranges(&mut acc, &r) {
            acc.push(r);
        }
        acc
    })
}

pub fn proccess_one(input: &str, line: i32) -> usize {
    let (_, p) = parse_input(input).unwrap();
    let ranges = p
        .iter()
        .flat_map(|(s, b)| s.line_intersections(&b, line))
        .collect::<Vec<_>>();

    let res = fold_rages(ranges);

    let on_line = p
        .iter()
        .filter(|(_, b)| b.1 == line)
        .map(|(_, b)| b)
        .collect::<HashSet<_>>();
    (res[0].start().abs() + res[0].end().abs()) as usize + 1 - on_line.len()
}

pub fn proccess_two(input: &str, search_space: i32) -> usize {
    let (_, p) = parse_input(input).unwrap();

    let (res_ranges, y_axis) = (0..=search_space)
        .find_map(|line| {
            let ranges = p
                .iter()
                .flat_map(|(s, b)| s.line_intersections_2(&b, line, search_space))
                .collect::<Vec<_>>();
            let res = fold_rages(ranges);
            if res.len() > 1 {
                Some((res, line))
            } else {
                None
            }
        })
        .unwrap();
    (res_ranges[1].start() - 1) as usize * 400_000_0 + y_axis as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part one: {}", proccess_one(&input, 2000000));
    }

    #[test]
    fn part_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part two: {}", proccess_two(&input, 400_000_0));
    }

    #[test]
    fn test_part_one() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(proccess_one(input, 10), 26);
    }

    #[test]
    fn test_part_two() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(proccess_two(input, 20), 56000011);
    }
}

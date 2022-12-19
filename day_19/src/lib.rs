use std::{
    cmp::{max, max_by},
    collections::{HashSet, VecDeque},
};

use nom::{
    bytes::complete::tag,
    character::{self, complete::line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore: u8,
    clay: u8,
    obsidian: (u8, u8),
    geode: (u8, u8),
    max_ore: u8,
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), character::complete::u8, tag(": "))(input)?;
    let (input, ore) = delimited(
        tag("Each ore robot costs "),
        character::complete::u8,
        tag(" ore. "),
    )(input)?;
    let (input, clay) = delimited(
        tag("Each clay robot costs "),
        character::complete::u8,
        tag(" ore. "),
    )(input)?;
    let (input, obsidian) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(
            character::complete::u8,
            tag(" ore and "),
            character::complete::u8,
        ),
        tag(" clay. "),
    )(input)?;
    let (input, geode) = delimited(
        tag("Each geode robot costs "),
        separated_pair(
            character::complete::u8,
            tag(" ore and "),
            character::complete::u8,
        ),
        tag(" obsidian."),
    )(input)?;

    Ok((
        input,
        Blueprint {
            id,
            ore,
            clay,
            obsidian,
            geode,
            max_ore: [ore, clay, obsidian.0, geode.0].into_iter().max().unwrap(),
        },
    ))
}

fn parse_blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(line_ending, parse_blueprint)(input)
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct State {
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: u8,
    geode_robot: u8,
    resources: Resources,
    time: u8,
}

impl State {
    fn tick(&mut self) {
        self.resources.ore += self.ore_robot;
        self.resources.clay += self.clay_robot;
        self.resources.obsidian += self.obsidian_robot;
        self.resources.geode += self.geode_robot;
        self.time += 1;
    }
}

#[derive(Debug, Clone, Copy)]
enum Resource {
    Geode,
    Obsidian,
    Clay,
    Ore,
    None,
}
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

fn bfs(b: &Blueprint, limit: u8) -> u8 {
    let state = State {
        ore_robot: 1,
        ..Default::default()
    };
    let mut q = VecDeque::from([state]);
    let mut geodes = 0;
    let mut seen_states = HashSet::new();

    while let Some(mut state) = q.pop_front() {
        geodes = geodes.max(state.resources.geode);
        if state.resources.geode < geodes.saturating_sub(1) || seen_states.contains(&state) {
            continue;
        };

        seen_states.insert(state);
        if state.time == limit {
            continue;
        }

        if state.resources.ore >= b.geode.0 && state.resources.obsidian >= b.geode.1 {
            let mut next_state = state;
            next_state.resources.ore -= b.geode.0;
            next_state.resources.obsidian -= b.geode.1;
            next_state.tick();
            next_state.geode_robot += 1;
            q.push_back(next_state);
        } else {
            if state.resources.ore >= b.ore && state.ore_robot < b.max_ore {
                let mut next_state = state;
                next_state.resources.ore -= b.ore;
                next_state.tick();
                next_state.ore_robot += 1;
                q.push_back(next_state);
            }
            if state.resources.ore >= b.clay {
                let mut next_state = state;
                next_state.resources.ore -= b.clay;
                next_state.tick();
                next_state.clay_robot += 1;
                q.push_back(next_state);
            }
            if state.resources.ore >= b.obsidian.0 && state.resources.clay >= b.obsidian.1 {
                let mut next_state = state;
                next_state.resources.ore -= b.obsidian.0;
                next_state.resources.clay -= b.obsidian.1;
                next_state.tick();
                next_state.obsidian_robot += 1;
                q.push_back(next_state);
            }
            state.tick();
            q.push_back(state);
        }
    }
    geodes
}

pub fn proccess_one(input: &str) -> usize {
    let (_, b) = parse_blueprints(input).unwrap();
    b.into_iter()
        .enumerate()
        .map(|(i, b)| bfs(&b, 24) as usize * (i + 1))
        .sum()
}
pub fn proccess_two(input: &str) -> usize {
    let (_, b) = parse_blueprints(input).unwrap();
    b.into_iter()
        .take(3)
        .map(|(b)| bfs(&b, 32) as usize)
        .product()
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
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(proccess_one(input), 33);
    }

    #[test]
    fn test_part_two() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(proccess_two(input), 56);
    }
}

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, line_ending},
    },
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
};

fn parse_connecting_valves(input: &str) -> IResult<&str, Vec<&str>> {
    alt((
        preceded(
            tag("; tunnels lead to valves "),
            separated_list1(tag(", "), character::complete::alpha1),
        ),
        preceded(tag("; tunnel leads to valve "), character::complete::alpha1).map(|s| vec![s]),
    ))(input)
}

#[derive(Debug)]
struct Valve<'a> {
    v: &'a str,
    flow_rate: u32,
    connecting_valves: Vec<&'a str>,
}

fn parse_line(input: &str) -> IResult<&str, Valve> {
    let (i, (v, flow_rate, connecting_valves)) = tuple((
        delimited(tag("Valve "), alpha1, tag(" has flow rate=")),
        character::complete::u32,
        parse_connecting_valves,
    ))(input)?;
    Ok((
        i,
        Valve {
            v,
            flow_rate,
            connecting_valves,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(line_ending, parse_line)(input)
}

fn bfs<'a>(start: &'a str, graph: &'a HashMap<&str, (u32, Vec<&str>)>) -> HashMap<&'a str, u32> {
    let mut q = VecDeque::from([(start, 0)]);
    let mut paths = HashMap::new();
    let mut visited = HashSet::from([start]);
    while let Some((current, minute)) = q.pop_front() {
        let (flow, valves) = graph.get(current).unwrap();
        if *flow != 0 && current != start {
            paths.insert(current, minute);
        }
        for valve in valves {
            if visited.insert(valve) {
                q.push_back((valve, minute + 1));
            }
        }
    }
    paths
}

fn dfs<'a>(
    mins: i32,
    start: &'a str,
    graph: &'a HashMap<&str, (u32, HashMap<&str, u32>)>,
    opend: &mut HashSet<&'a str>,
) -> u32 {
    if mins <= 0 {
        opend.remove(start);
        return 0;
    }
    let (flow, valves) = graph.get(start).unwrap();
    let flow = flow * mins as u32;
    opend.insert(start);
    let max_flow = valves
        .iter()
        .flat_map(|(valve, cost)| {
            if !opend.contains(valve) {
                Some(dfs((mins - 1) - *cost as i32, valve, graph, opend))
            } else {
                None
            }
        })
        .max()
        .unwrap_or_default();
    opend.remove(start);
    flow + max_flow
}

fn dfs2<'a>(
    mins: i32,
    mins1: i32,
    start: &'a str,
    elefant: &'a str,
    graph: &'a HashMap<&str, (u32, HashMap<&str, u32>)>,
    opend: &mut HashSet<&'a str>,
) -> u32 {
    match (mins, mins1) {
        (i32::MIN..=0, i32::MIN..=0) => {
            opend.remove(start);
            opend.remove(elefant);
            // println!("bothend:  start: {start}, elefant: {elefant}");
            0
        }
        (i32::MIN..=0, _) => {
            // println!("start end:  start: {start}, elefant: {elefant}");
            let a = dfs(mins1, elefant, graph, opend);
            opend.remove(start);
            a
        }
        (_, i32::MIN..=0) => {
            // println!("elefant end:  start: {start}, elefant: {elefant}");
            let a = dfs(mins, start, graph, opend);
            opend.remove(elefant);
            a
        }
        _ => {
            let (flow, valves) = graph.get(start).unwrap();
            let (flow_e, valves_e) = graph.get(elefant).unwrap();
            let flow = flow * mins as u32 + flow_e * mins1 as u32;

            // opend.insert(start);
            // opend.insert(elefant);
            let max_flow = valves
                .iter()
                .cartesian_product(valves_e)
                .filter(|(a, b)| a.0 != b.0)
                .flat_map(|((valve, cost), (valve2, cost2))| {
                    if !opend.contains(valve) && !opend.contains(valve2) {
                        opend.insert(valve);
                        opend.insert(valve2);
                        let x = Some(dfs2(
                            (mins - 1) - *cost as i32,
                            (mins1 - 1) - *cost2 as i32,
                            valve,
                            valve2,
                            graph,
                            opend,
                        ));
                        opend.remove(valve);
                        opend.remove(valve2);
                        x
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or_default();
            opend.remove(start);
            opend.remove(elefant);
            flow + max_flow
        }
    }
}

pub fn proccess_one(input: &str) -> u32 {
    let (_, a) = parse_input(input).unwrap();
    let graph = a
        .into_iter()
        .map(|v| (v.v, (v.flow_rate, v.connecting_valves)))
        .collect::<HashMap<_, _>>();
    let c = bfs("AA", &graph);
    let mut a = graph
        .iter()
        .filter(|(_, (v, _))| *v != 0)
        .map(|(k, (flow, _))| (*k, (*flow, bfs(k, &graph))))
        .collect::<HashMap<_, _>>();

    a.insert("AA", (0, c));
    dfs(30, "AA", &a, &mut HashSet::new())
}

pub fn proccess_two(input: &str) -> u32 {
    let (_, a) = parse_input(input).unwrap();
    let graph = a
        .into_iter()
        .map(|v| (v.v, (v.flow_rate, v.connecting_valves)))
        .collect::<HashMap<_, _>>();
    let c = bfs("AA", &graph);
    let mut a = graph
        .iter()
        .filter(|(_, (v, _))| *v != 0)
        .map(|(k, (flow, _))| (*k, (*flow, bfs(k, &graph))))
        .collect::<HashMap<_, _>>();
    a.insert("AA", (0, c));
    dfs2(26, 26, "AA", "AA", &a, &mut HashSet::from(["AA"]))
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
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(proccess_one(input), 1651);
    }

    #[test]
    fn test_part_two() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(proccess_two(input), 1707);
    }
}

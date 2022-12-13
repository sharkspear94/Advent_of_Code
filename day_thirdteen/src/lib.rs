use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::line_ending},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult, Parser,
};

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        character::complete::u32.map(|a| Packet::Num(a as usize)),
        delimited(
            tag("["),
            separated_list0(tag(","), parse_packet).map(|a| Packet::List(a)),
            tag("]"),
        ),
    ))(input)
}

fn parse_packet_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(parse_packet, line_ending, parse_packet)(input)
}

fn parse_packets(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(pair(line_ending, line_ending), parse_packet_pair)(input)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Num(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.partial_cmp(r),
            (Packet::Num(l), Packet::List(r)) => [Packet::Num(*l)][..].partial_cmp(&&r[..]),
            (Packet::List(l), Packet::Num(r)) => l[..].partial_cmp(&[Packet::Num(*r)]),
            (Packet::List(l), Packet::List(r)) => l.partial_cmp(r),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn proccess_one(input: &str) -> usize {
    let (_, packets) = parse_packets(input).unwrap();
    packets
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| l.le(r))
        .map(|(i, _)| i + 1)
        .sum()
}
pub fn proccess_two(input: &str) -> usize {
    let (_, packets) = parse_packets(input).unwrap();

    let diveder_packet1 = parse_packet("[[2]]").unwrap().1;
    let diveder_packet2 = parse_packet("[[6]]").unwrap().1;
    let mut packets = packets
        .iter()
        .flat_map(|(l, r)| [l, r])
        .chain([&diveder_packet1, &diveder_packet2])
        .collect::<Vec<_>>();
    packets.sort();

    (packets.binary_search(&&diveder_packet1).unwrap() + 1)
        * (packets.binary_search(&&diveder_packet2).unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_asd() {
        assert!([1, 2, 3, 4][..].lt(&[5][..]))
    }

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
        let input = std::fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(proccess_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(proccess_two(&input), 140);
    }
}

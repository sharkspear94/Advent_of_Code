#![feature(iter_array_chunks)]
use std::cmp::Ordering;

use nom::{IResult, branch::alt, sequence::{delimited, separated_pair, pair}, bytes::complete::{tag}, multi::{separated_list0, separated_list1}, Parser, character::{self, complete::line_ending}, combinator::eof};

fn parse_packet(input: &str) -> IResult<&str,Packet> {
    alt((
        character::complete::u32.map(|a| Packet::Num(a as usize)),
        delimited(tag("["), separated_list0(tag(","),parse_packet).map(|a| Packet::List(a)), tag("]"))
    ))(input)
}

fn parse_packet_pair(input: &str) -> IResult<&str,(Packet,Packet)> {
    separated_pair(parse_packet, line_ending, parse_packet)(input)
}

fn parse_packets(input: &str) -> IResult<&str,Vec<(Packet,Packet)>> {
    separated_list1(pair(line_ending,line_ending),parse_packet_pair)(input)
}

#[derive(Debug,PartialEq,Eq,Clone)]
enum Packet {
    Num(usize),
    List(Vec<Packet>)
}

impl PartialOrd for Packet{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self,other) {
            (Packet::Num(l), Packet::Num(r)) => l.partial_cmp(r),
            (Packet::Num(l) , Packet::List(r)) => vec![Packet::Num(*l)].partial_cmp(&r),
            (Packet::List(l), Packet::Num(r)) => l.partial_cmp(&vec![Packet::Num(*r)]),
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
    let (_,packets) = parse_packets(input).unwrap();
    packets.iter()
        .enumerate()
        .filter(|(_,(l,r))|l.le(r))
        .map(|(i,_)|i+1)
        .sum()
}
pub fn proccess_two(input: &str) -> usize {
    let (_,packets) = parse_packets(input).unwrap();
    let mut packets =packets.iter()
        .flat_map(|(l,r)| [l,r])
        .collect::<Vec<_>>();
    let diveder_packet1 = parse_packet("[[2]]").unwrap().1;
    let diveder_packet2 = parse_packet("[[6]]").unwrap().1;
    packets.push(&diveder_packet1);
    packets.push(&diveder_packet2);
    packets.sort();

    packets.iter()
        .enumerate()
        .filter(|(_,p)| **p==&diveder_packet1 || **p==&diveder_packet2)
        .map(|(i,_)|i+1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part one: {}",proccess_one(&input) );
    }

    #[test]
    fn part_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part two: {}",proccess_two(&input) );
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

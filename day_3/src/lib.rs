#![feature(iter_array_chunks)]
use std::collections::HashSet;

pub fn procces_one(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let left: HashSet<char> = HashSet::from_iter(line[..line.len()/2].chars());
            let right = HashSet::from_iter(line[line.len()/2..].chars());
            left.intersection(&right).copied().next().unwrap()
        }).map(|c| if c.is_lowercase() {
            c as u32 - 'a' as u32 + 1
        } else {
            c as u32 - 'A' as u32 + 1 + 26
        })
        .sum()
}

pub fn procces_two(input: &str) -> u32 {
    input.lines()
        .array_chunks()
        .map(|[f,m,l]| {
            let first: HashSet<char> = HashSet::from_iter(f.chars());
            let middle = HashSet::from_iter(m.chars());
            let last: HashSet<char> = HashSet::from_iter(l.chars());

            let intersection = first.intersection(&middle).copied().collect();
            last.intersection(&intersection).copied().next().unwrap()
        })
        .map(|c| if c.is_lowercase() {
            c as u32 - 'a' as u32 + 1
        } else {
            c as u32 - 'A' as u32 + 1 + 26
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result Part one: {}",(procces_one(&input)));
    }

    #[test]
    fn part_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result Part two: {}",(procces_two(&input)));
    }
    #[test]
    fn it_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(procces_one(input), 157);
    }

    #[test]
    fn it_works_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(procces_two(input), 70);
    }
}

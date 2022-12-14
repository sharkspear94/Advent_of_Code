use nom::{IResult, character::{self, complete::{line_ending, not_line_ending}}, bytes::complete::{tag, take}, sequence::{tuple, delimited, terminated}, multi::{separated_list1}, branch::alt};

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    times: usize
}

fn parse_move(input: &str) -> IResult<&str,Move> {
    let (i,(_,times,_,from,_,to)) = tuple((tag("move "), character::complete::u32, tag(" from "),character::complete::u32,tag(" to "),character::complete::u32))(input)?;
    Ok((i,Move {
            from: from as usize -1,
            to: to as usize -1,
            times: times as usize,
        }))
}
fn parse_box(input: &str) -> IResult<&str,&str> {
    delimited(tag("["), take(1usize) , tag("]"))(input)
}

fn parse_box_opt(input: &str) -> IResult<&str,Option<&str>> {
    let (i,b) = alt((parse_box,tag("   ")))(input)?;
    match b {
        "   " => Ok((i,None)),
        b => Ok((i,Some(b)))
    }
}

fn parse_box_line(input: &str) -> IResult<&str,Vec<Option<&str>>> {
    separated_list1(tag(" "),parse_box_opt)(input)
}

fn read_line(input: &str) -> IResult<&str,&str> {
    terminated(not_line_ending,line_ending)(input)
}

pub fn procces_one(input: &str) -> String {
    let (i, crates) = separated_list1(line_ending,parse_box_line)(input).unwrap();
    let (i,_) = read_line(i).unwrap();
    let (i,_) = read_line(i).unwrap();
    let (i,_) = read_line(i).unwrap();
    let (_, moves) = separated_list1(line_ending,parse_move)(i).unwrap();
    let len = crates[0].len();
    let mut crates = crates.into_iter()
        .rev()
        .fold(vec![vec![];len],|mut stack, a| {
            a.into_iter()
                .enumerate()
                .filter_map(|(i,b)| b.map(|a|(i,a)))
                .for_each(|(i,crate_)| stack[i].push(crate_)); 
            stack
        });
    for Move{from,to,times} in moves {
        let len = crates[from].len();
        let d = crates[from].drain(len-times..).collect::<Vec<_>>();
        crates[to].extend(d.into_iter().rev());
    }
    crates.into_iter()
        .filter_map(|a| a.last().copied())
        .collect()
}

pub fn procces_two(input: &str) -> String {
    let (i, crates) = separated_list1(line_ending,parse_box_line)(input).unwrap();
    let (i,_) = read_line(i).unwrap();
    let (i,_) = read_line(i).unwrap();
    let (i,_) = read_line(i).unwrap();
    let (_, moves) = separated_list1(line_ending,parse_move)(i).unwrap();
    let len = crates[0].len();
    let mut crates = crates.into_iter()
        .rev()
        .fold(vec![vec![];len],|mut stack, a| {
            a.into_iter()
                .enumerate()
                .filter_map(|(i,b)| b.map(|a|(i,a)))
                .for_each(|(i,crate_)| stack[i].push(crate_)); 
            stack
        });
    for Move{from,to,times} in moves {
        let len = crates[from].len();
        let d = crates[from].drain(len-times..).collect::<Vec<_>>();
        crates[to].extend(d);
    }
    crates.into_iter()
        .filter_map(|a| a.last().copied())
        .collect()
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
    fn it_works_1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(procces_one(input), "CMZ");
    }

    #[test]
    fn it_works_2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(procces_two(input), "MCD");
    }
}

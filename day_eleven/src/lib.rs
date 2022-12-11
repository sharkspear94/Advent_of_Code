use nom::{IResult, sequence::{terminated, preceded, delimited, tuple, pair}, character::{complete::{not_line_ending, line_ending}, self}, bytes::complete::{tag, take_till}, multi::{separated_list1, separated_list0}, branch::alt, Parser, combinator::eof};


struct Monkey {
    items: Vec<usize>,
    inspected: usize,
    divident: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    test_worry: Box<dyn Fn(usize) -> usize>,
}


fn read_line(input: &str) -> IResult<&str,&str> {
    terminated(not_line_ending,line_ending)(input)
}

fn parse_starting_items(input: &str) -> IResult<&str,Vec<usize>> {
    delimited(tag("  Starting items: "),separated_list0(tag(", "),character::complete::u32).map(|u| u.into_iter().map(|a| a as usize).collect()),line_ending)(input)
}

fn parse_operation(input: &str) -> IResult<&str,Box<dyn Fn(usize) -> usize>> {
    let (input,(op,operand)) = preceded(tag("new = old "),pair(alt((tag("* "),tag("+ "))),take_till(|c| c == '\n' || c == '\r')))(input)?;
    match (op,operand) {
        ("* ","old") => Ok((input,Box::new(|old| old * old))),
        ("+ ","old") => Ok((input,Box::new(|old| old + old))),
        ("* ",l) => {
            let literal = l.parse::<usize>().expect("lieral parse failed");
            Ok((input,Box::new(move |old| old * literal)))
        }
        ("+ ",l) => {
            let literal = l.parse::<usize>().expect("lieral parse failed");
            Ok((input,Box::new(move |old| old + literal)))
        }
        _ => panic!("parsing fn panic")
    }
}

fn parse_operation_line(input: &str) -> IResult<&str,Box<dyn Fn(usize) -> usize>> {
    delimited(tag("  Operation: "),parse_operation,line_ending)(input)
}

fn parse_test_worry(input: &str) -> IResult<&str,(usize,Box<dyn Fn(usize) -> usize>)> {
    let (input,divident) = delimited(tag("  Test: divisible by "),character::complete::u32,line_ending)(input)?;
    let (input,monkey_true) = delimited(tag("    If true: throw to monkey "),character::complete::u32,line_ending)(input)?;
    let (input,monkey_false) = delimited(tag("    If false: throw to monkey "),character::complete::u32,alt((line_ending,eof)))(input)?;
    Ok((input,(divident as usize,Box::new(move |worry| if worry % divident as usize == 0 {monkey_true as usize} else { monkey_false as usize}))))
}

fn parse_monkey(input: &str) -> IResult<&str,Monkey> {
    let (input,_) = read_line(input)?;
    let (input,items) = parse_starting_items(input)?;
    let (input,operation) = parse_operation_line(input)?;
    let (input,(divident,test_worry)) = parse_test_worry(input)?;
    Ok((input,Monkey { items,inspected: 0, operation, test_worry,divident }))
}

fn parse_monkeys(input: &str) -> IResult<&str,Vec<Monkey>> {
    separated_list1(line_ending, parse_monkey)(input)
}

pub fn process_one(input: &str) -> usize {
    let (_,mut monkeys) = parse_monkeys(input).unwrap();
    let mut buffers = vec![vec![];monkeys.len()];
    for _ in 0..20 {
        for (i,monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.extend(buffers[i].drain(..));
            monkey.inspected += monkey.items.drain(..)
                .map(|worry| (monkey.operation)(worry) / 3)
                .map(|new_worry| buffers[(monkey.test_worry)(new_worry)].push(new_worry))
                .count();
        }
        buffers.iter_mut()
            .enumerate()
            .for_each(|(i,buf)| monkeys[i].items.extend(buf.drain(..)))
    }
    monkeys.sort_by(|a,b| b.inspected.cmp(&a.inspected));
    
    monkeys.iter()
        .take(2)
        .map( |m| m.inspected )
        .product()

}

pub fn process_two(input: &str) -> usize {
    let (_,mut monkeys) = parse_monkeys(input).unwrap();
    let mut buffers = vec![vec![]; monkeys.len()];
    let common_div: usize = monkeys.iter().map(|m|m.divident).product();
    for _ in 0..10000 {
        for (i,monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.extend(buffers[i].drain(..));
            monkey.inspected += monkey.items.drain(..)
                .map(|worry| (monkey.operation)(worry) % common_div)
                .map(|new_worry| buffers[(monkey.test_worry)(new_worry)].push(new_worry))
                .count();
        }
        buffers.iter_mut()
            .enumerate()
            .for_each(|(i,buf)| monkeys[i].items.extend(buf.drain(..)))
    }
    monkeys.sort_by(|a,b| b.inspected.cmp(&a.inspected));
    monkeys.into_iter()
        .take(2)
        .map(|m|m.inspected)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part one: {}",process_one(&input));
    }

    #[test]
    fn part_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part two: {}",process_two(&input));
    }

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(process_one(&input), 10605);
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(process_two(&input), 2713310158);
    }
}

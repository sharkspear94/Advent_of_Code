use std::iter;

use nom::{IResult, bytes::complete::tag, character::{self, complete::line_ending}, multi::separated_list1, branch::alt};

fn parse_add_x(input: &str) -> IResult<&str,Instruction> {
    let (input,_) = tag("addx ")(input)?;
    let (i,val) = character::complete::i32(input)?;
    Ok((i,Instruction::AddX(val)))
}
fn parse_noop(input: &str) -> IResult<&str,Instruction> {
    let (i,_) = tag("noop")(input)?;
    Ok((i,Instruction::NoOp))
}
fn parse_instruction(input: &str) -> IResult<&str,Vec<Instruction>> {
    separated_list1(line_ending, alt((parse_add_x,parse_noop)))(input)
}

enum Instruction {
    NoOp,
    AddX(i32),
}

pub fn procces_part_one(input: &str) -> i32 {
    let (_,instructions) = parse_instruction(input).unwrap();
    let mut x = 1; 
    instructions.into_iter()
        .flat_map(|i| match i {
            Instruction::NoOp => iter::repeat(x).take(1),
            Instruction::AddX(val) => {x+=val; iter::repeat(x-val).take(2)},
        })
        .enumerate()
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(cycle,value)| (cycle+1) as i32 * value )
        .sum()
}

pub fn procces_part_two(input: &str) -> String {
    let (_,instructions) = parse_instruction(input).unwrap();
    let crt_flattend = instructions.into_iter()
        .scan(1,|x,i| match i {
            Instruction::NoOp => Some(iter::repeat(*x).take(1)),
            Instruction::AddX(val) => {*x+=val; Some(iter::repeat(*x-val).take(2))},
        })
        .flatten()
        .enumerate()
        .fold(['.';40*6],|mut crt, (cycle, x)| {
            let sprite = x+(40*(cycle as i32/40));
            if (cycle as i32) >= sprite-1 && (cycle as i32) <= sprite+1 {
                crt[cycle] = '#';
            }
            crt
        });
    crt_flattend.chunks_exact(40)
        .enumerate()
        .fold([['\n';41];6],|mut crt,(i,row)| {
            crt[i][..40].copy_from_slice(row);
            crt
        }).into_iter()
        .flatten()
        .collect()


    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result Part One:{}",procces_part_one(&input));
    }


    #[test]
    fn part_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result Part two: \n{}",procces_part_two(&input));
    }

    #[test]
    fn it_works_part_1() {
    let input = std::fs::read_to_string("./test_input.txt").unwrap(); 
        assert_eq!(procces_part_one(&input), 13140);
    }

    #[test]
    fn it_works_part_2() {
        let input = std::fs::read_to_string("./test_input.txt").unwrap(); 
        let res = String::from("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
        assert_eq!(procces_part_two(&input), res);
    }
}

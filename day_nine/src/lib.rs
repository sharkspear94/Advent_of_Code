use std::collections::HashSet;

use nom::{IResult, multi::separated_list1, character::{complete::{alpha1, line_ending}, self}, sequence::tuple, bytes::complete::tag};

#[derive(Debug)]
enum Step {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

fn parse_step(input: &str) -> IResult<&str,Step> {
    let (i,(direction,_,len)) = tuple((alpha1,tag(" "),character::complete::u32))(input)?;
    let step = match direction {
        "R" => Step::Right(len as usize),
        "L" => Step::Left(len as usize),
        "U" => Step::Up(len as usize),
        "D" => Step::Down(len as usize),
        rest => panic!("step parser faild with: {}", rest)
    };

    Ok((i,step))
}

fn parse_steps(input: &str) -> IResult<&str,Vec<Step>> {
    separated_list1(line_ending, parse_step)(input)
}

pub fn process_part_one(input: &str) -> usize {

    let (_,steps) = parse_steps(input).expect("faild to parse input");
    let mut visited = HashSet::new();
        
    let mut head = (0,0);
    let mut current_tail = (0,0);
    for step in steps {
        match step {
            Step::Left(s) => for _ in 0..s {
                head.0 -= 1;
                current_tail = next_follow(current_tail,head);
                visited.insert(current_tail);
            },
            Step::Right(s) => for _ in 0..s {
                head.0 += 1;
                current_tail = next_follow(current_tail,head);
                visited.insert(current_tail);
            },
            Step::Up(s) => for _ in 0..s {
                head.1 += 1;
                current_tail = next_follow(current_tail,head);
                visited.insert(current_tail);
            },
            Step::Down(s) => for _ in 0..s {
                head.1 -= 1;
                current_tail = next_follow(current_tail,head);
                visited.insert(current_tail);
            },
        }
    }
    
    visited.len()
}

pub fn process_part_two(input: &str) -> usize {

    let (_,steps) = parse_steps(input).expect("faild to parse input");
    let mut visited = HashSet::new();
    let mut rope = [(1001,1000);10];
    for step in steps {
        match step {
            Step::Left(s) => for _ in 0..s {
                rope[0].0 -= 1;
                for i in 1..10usize {
                    rope[i] = next_follow(rope[i],rope[i-1]);
                }
                visited.insert(rope[9]);
                            },
            Step::Right(s) => for _ in 0..s {
                rope[0].0 += 1;
                for i in 1..10usize {
                    rope[i] = next_follow(rope[i],rope[i-1]);
                }
                visited.insert(rope[9]);
                            },
            Step::Up(s) => for _ in 0..s {
                rope[0].1 += 1;
                for i in 1..10usize {
                    rope[i] = next_follow(rope[i],rope[i-1]);
                }
                visited.insert(rope[9]);
                            },
            Step::Down(s) => for _ in 0..s {
                rope[0].1 -= 1;
                for i in 1..10usize {
                    rope[i] = next_follow(rope[i],rope[i-1]);   
                }
                visited.insert(rope[9]);
            },
        }
    }
    visited.len()
}


fn next_follow((current_x,current_y): (i32,i32), (next_head_x,next_head_y): (i32,i32)) -> (i32,i32) {
    let x_diff = current_x - next_head_x;
    let y_diff = current_y - next_head_y;
    if x_diff.abs() < 2 && y_diff.abs() < 2 {
        return (current_x,current_y)
    }
    if y_diff.abs() < 2 {
        (next_head_x+x_diff.signum(),next_head_y)
    } else if x_diff.abs() < 2 {
        (next_head_x,next_head_y+y_diff.signum())
    } else {
        (next_head_x+x_diff.signum(),next_head_y+y_diff.signum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_step() {
        assert_eq!(next_follow((0,0),(2,0)),((1,0)));
        assert_eq!(next_follow((0,0),(-2,0)),((-1,0)));
        assert_eq!(next_follow((2,0),(0,0)),((1,0)));
        assert_eq!(next_follow((0,0),(0,2)),((0,1)));
        assert_eq!(next_follow((0,2),(0,0)),((0,1)));
        assert_eq!(next_follow((0,0),(1,1)),((0,0)));
        assert_eq!(next_follow((0,0),(1,0)),((0,0)));
        assert_eq!(next_follow((0,0),(0,1)),((0,0)));
        assert_eq!(next_follow((0,0),(-1,0)),((0,0)));
        assert_eq!(next_follow((0,0),(0,-1)),((0,0)));
        assert_eq!(next_follow((0,0),(1,2)),((1,1)));
        assert_eq!(next_follow((0,0),(2,1)),((1,1)));
        assert_eq!(next_follow((2,1),(0,0)),((1,0)));
        assert_eq!(next_follow((1,2),(0,0)),((0,1)));
        assert_eq!(next_follow((0,0),(2,2)),((1,1)));
    }

    #[test]
    fn part_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result Part 1: {}",process_part_one(&input))
    }


    #[test]
    fn it_works_part_one() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(process_part_one(input), 13);
    }


    #[test]
    fn part_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result Part 2: {}",process_part_two(&input))
    }

    #[test]
    fn it_works_part_two_small() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(process_part_two(input), 1);
    }

    #[test]
    fn it_works_part_two() {
let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(process_part_two(input), 36);
    }
}

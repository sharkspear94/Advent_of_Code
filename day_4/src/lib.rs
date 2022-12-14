use nom::{IResult, character::{self, complete::line_ending}, sequence::separated_pair, bytes::complete::tag, multi::{separated_list0, separated_list1}};


fn parse_range(input: &str) -> IResult<&str,(u32,u32)> {
    separated_pair(character::complete::u32, tag("-"), character::complete::u32)(input)
}

fn parse_range_pair(input: &str) -> IResult<&str,((u32,u32),(u32,u32))> {
    separated_pair(parse_range, tag(","), parse_range)(input)
}


fn procces_one(input: &str) -> usize {
    let (i,ranges) = separated_list1(line_ending,parse_range_pair )(input).unwrap();

    ranges.iter()
        .filter(|(l_r,r_r)| (l_r.0 <= r_r.0 && l_r.1 >= r_r.1) || (r_r.0 <= l_r.0 && r_r.1 >= l_r.1))
        .count()
}

fn procces_two(input: &str) -> usize {
    let (i,ranges) = separated_list1(line_ending,parse_range_pair )(input).unwrap();

    ranges.iter()
        .filter(|(l_r,r_r)| !((l_r.0 > r_r.1) || (l_r.1 < r_r.0)))
        .count()
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
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(procces_one(input), 2);
    }

    #[test]
    fn it_works_2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(procces_two(input), 4);
    }
}

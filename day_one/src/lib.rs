use nom::{multi::{separated_list1, count}, character::{complete::line_ending, self}, IResult};


fn parse_elf(input: &str) -> IResult<&str,Vec<u32>> {
    separated_list1(line_ending, character::complete::u32)(input)
}

fn parse_elfs(input: &str) -> IResult<&str,Vec<Vec<u32>>> {
    separated_list1(count(line_ending,2), parse_elf)(input)
}

fn part_one(input: &str) -> String {
    let (_,elfs) = parse_elfs(input).unwrap();
    let a = elfs.into_iter()
        .map(|l| {
            l.iter()
                .sum::<u32>()
        }
        )
        .max()
        .unwrap();
    a.to_string()
}

fn part_two(input: &str) -> String {
    let (_,elfs) = parse_elfs(input).unwrap();
    let mut a = elfs.into_iter()
        .map(|l| {
            l.iter()
                .sum::<u32>()
        }
        ).collect::<Vec<_>>();
    a.sort();
    a.iter()
        .rev()
        .take(3)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let f = std::fs::read_to_string("./input.txt").unwrap();
        let res = part_one(&f);
        println!("{res}");
    }

    #[test]
    fn test_part_two() {
        let f = std::fs::read_to_string("./input.txt").unwrap();
        let res = part_two(&f);
        println!("{res}");
    }

    #[test]
    fn test_example() {
        let example = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(&part_one(example),"24000");
    }
}

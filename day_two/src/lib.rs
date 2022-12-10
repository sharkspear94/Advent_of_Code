pub fn procces_one(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let mut it = line.split_terminator(' ');
            let a = it.next().unwrap();
            let b = it.next().unwrap();
            match (a,b) {
                ("A","X") => 3 + 1,
                ("A","Y") => 6 + 2,
                ("A","Z") => 0 + 3,
                ("B","X") => 0 + 1,
                ("B","Y") => 3 + 2,
                ("B","Z") => 6 + 3,
                ("C","X") => 6 + 1,
                ("C","Y") => 0 + 2,
                ("C","Z") => 3 + 3,
                _ => panic!("wrong input")
            }
        }).sum()
}

pub fn procces_two(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let mut it = line.split_terminator(' ');
            let a = it.next().unwrap();
            let b = it.next().unwrap();
            match (a,b) {
                ("A","X") => 0 + 3,
                ("A","Y") => 3 + 1,
                ("A","Z") => 6 + 2,
                ("B","X") => 0 + 1,
                ("B","Y") => 3 + 2,
                ("B","Z") => 6 + 3,
                ("C","X") => 0 + 2,
                ("C","Y") => 3 + 3,
                ("C","Z") => 6 + 1,
                _ => panic!("wrong input")
            }
        }).sum()
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
        let input = "A Y
B X
C Z";
        assert_eq!(procces_one(input), 15);
    }
}

use itertools::Itertools;

pub fn process_part_one(input: &str) -> usize {
    let str_c = input.chars()
        .collect::<Vec<_>>();
    let res = str_c.windows(4)
        .enumerate()
        .filter(|(_,c)| c.iter().all_unique() )
        .next()
        .unwrap();
    res.0 + 4
}

pub fn process_part_two(input: &str) -> usize {
    let str_c = input.chars()
        .collect::<Vec<_>>();
    let res = str_c.windows(14)
        .enumerate()
        .filter(|(_,c)| c.iter().all_unique() )
        .next()
        .unwrap();
    res.0 + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("./input.txt").unwrap();

        println!("part one: {}",process_part_one(&input));
    }

    #[test]
    fn it_works() {
        assert_eq!(process_part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(process_part_one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(process_part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(process_part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("./input.txt").unwrap();

        println!("part two: {}",process_part_two(&input));
    }

    #[test]
    fn it_works2() {
        assert_eq!(process_part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(process_part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(process_part_two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(process_part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(process_part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

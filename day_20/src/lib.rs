pub fn process1(input: &str) -> i64 {
    let nums = input
        .lines()
        .flat_map(|num| num.parse::<i64>())
        .enumerate()
        .collect::<Vec<_>>();
    let mut res = Vec::from_iter(nums.clone());
    let message_size = nums.len() as i64 - 1;
    nums.iter().for_each(|&(i, num)| {
        let index = res.iter().position(|&(idx, _)| idx == i).unwrap();
        let mut new_index = index as i64 + num;
        new_index = ((new_index % message_size) + message_size) % message_size;
        let number = res.remove(index);
        res.insert(new_index as usize, number);
    });
    let pos = res.iter().position(|a| a.1 == 0).unwrap();
    let a = res.iter().cycle().skip(pos).nth(1000).unwrap().1;
    let b = res.iter().cycle().skip(pos).nth(2000).unwrap().1;
    let c = res.iter().cycle().skip(pos).nth(3000).unwrap().1;
    a + b + c
}

pub fn process2(input: &str) -> i64 {
    let nums = input
        .lines()
        .flat_map(|num| num.parse::<i64>())
        .map(|num| num * 811589153)
        .enumerate()
        .collect::<Vec<_>>();
    let mut res = Vec::from_iter(nums.clone());
    let message_size = nums.len() as i64 - 1;
    for _ in 0..10 {
        nums.iter().for_each(|&(i, num)| {
            let index = res.iter().position(|&(idx, _)| idx == i).unwrap();
            let mut new_index = index as i64 + num;
            new_index = ((new_index % message_size) + message_size) % message_size;
            let number = res.remove(index);
            res.insert(new_index as usize, number);
        });
    }
    let pos = res.iter().position(|a| a.1 == 0).unwrap();
    let a = res.iter().cycle().skip(pos).nth(1000).unwrap().1;
    let b = res.iter().cycle().skip(pos).nth(2000).unwrap().1;
    let c = res.iter().cycle().skip(pos).nth(3000).unwrap().1;
    a + b + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        let result = process1(&input);
        println!("Result part one: {result}")
    }

    #[test]
    fn process_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        let result = process2(&input);
        println!("Result part two: {result}")
    }

    #[test]
    fn process_one_test() {
        let input = "1
2
-3
3
-2
0
4";
        let result = process1(input);
        assert_eq!(result, 3)
    }

    #[test]
    fn process_two_test() {
        let input = "1
2
-3
3
-2
0
4";
        let result = process2(input);
        assert_eq!(result, 1623178306)
    }
}

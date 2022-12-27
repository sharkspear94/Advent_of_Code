fn sum(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.chars().rev().enumerate().fold(0i64, |acc, (i, c)| {
                let i = i as u32;
                match c {
                    '=' => acc + -2 * 5i64.pow(i),
                    '-' => acc + -1 * 5i64.pow(i),
                    '0' => acc + 0 * 5i64.pow(i),
                    '1' => acc + 1 * 5i64.pow(i),
                    '2' => acc + 2 * 5i64.pow(i),
                    rest => panic!("{rest} is not a valid char"),
                }
            })
        })
        .sum()
}

fn snafu_output(num: i64) -> String {
    let mut num = num;
    let s = std::iter::from_fn(move || {
        if num == 0 {
            None
        } else {
            match num % 5 {
                0 => {
                    num /= 5;
                    Some('0')
                }
                1 => {
                    num -= 1;
                    num /= 5;
                    Some('1')
                }
                2 => {
                    num -= 2;
                    num /= 5;
                    Some('2')
                }
                3 => {
                    num += 2;
                    num /= 5;
                    Some('=')
                }
                4 => {
                    num += 1;
                    num /= 5;
                    Some('-')
                }
                _ => panic!(),
            }
        }
    })
    .collect::<String>();
    s.chars().rev().collect()
}

pub fn process1(input: &str) -> String {
    let num = sum(input);
    snafu_output(num)
}

pub fn process2(input: &str) -> String {
    todo!()
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
    fn process_one_sum_test() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        let result = sum(input);
        assert_eq!(result, 4890)
    }

    #[test]
    fn process_one_test() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        let result = process1(input);
        assert_eq!(result, "2=-1=0")
    }

    #[test]
    fn process_two_test() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        let result = process2(input);
        assert_eq!(result, "20")
    }
}

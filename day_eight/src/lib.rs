fn process_one(input: &str) -> String {
    let width = input.chars().take_while(|c|!c.is_ascii_whitespace()).count();

    let v = input.lines()
        .flat_map(|line| line.chars()).filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>();
    let seen = v.iter()
        .enumerate()
        .map(|(i,c)| traverse(c, i, width, &v[..]))
        .filter(|a| *a)
        .count();
    seen.to_string()
}

fn traverse(c: &char,idx: usize,len: usize, s: &[char]) -> bool {
    let right_bound = (((idx/len)+1)*len) -1;
    let left_bound = idx/len * len;
    right(c,&s[idx..=right_bound]) || left(c, &s[left_bound..idx])
     || up(c, len, &s[..=idx]) || down(c, len, &s[idx..]) 
}

fn right(c: &char, s: &[char]) -> bool {
    !s.iter().skip(1).any(|tree| tree >= c)
}

fn left(c: &char, s: &[char]) -> bool {
    !s.iter().rev().any(|tree| tree >= c)
}

fn up(c: &char,len: usize, s: &[char]) -> bool {
    !s.iter().rev().step_by(len).skip(1).any(|tree| tree >= c)
}

fn down(c: &char,len: usize, s: &[char]) -> bool {
    !s.iter().step_by(len).skip(1).any(|tree| tree >= c)
}


// ------------------------------------------------------------------

fn process_two(input: &str) -> String {
    let width = input.chars().take_while(|c|!c.is_ascii_whitespace()).count();
    
    let v = input.lines()
        .flat_map(|line| line.chars()).filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>();
    let seen = v.iter()
        .enumerate()
        .map(|(i,c)| traverse2(c, i, width, &v[..]))
        // .inspect(|a| println!("{a}"))
        .max().unwrap();
    seen.to_string()
}

fn traverse2(c: &char,idx: usize,len: usize, s: &[char]) -> usize {
    let right_bound = (((idx/len)+1)*len) -1;
    let left_bound = idx/len * len;
    right2(c,&s[idx..=right_bound]) * left2(c, &s[left_bound..idx])
     * up2(c, len, &s[..=idx]) * down2(c, len, &s[idx..]) 
}

fn right2(c: &char, s: &[char]) -> usize {
    let c = s.iter().skip(1).take_while(|tree| tree < &c).count();
    if s.iter().skip(1).skip(c).next().is_none() {
        c
    } else {c+1}
}

fn left2(c: &char, s: &[char]) -> usize {
    let c = s.iter().rev().take_while(|tree| tree < &c).count();
    if s.iter().rev().skip(c).next().is_none() {
        c
    } else {c+1}
}

fn up2(c: &char,len: usize, s: &[char]) -> usize {
    let c = s.iter().rev().step_by(len).skip(1).take_while(|tree| tree < &c).count();
    if s.iter().rev().step_by(len).skip(1).skip(c).next().is_none() {
        c
    } else {c+1}
}

fn down2(c: &char,len: usize, s: &[char]) -> usize {
    let c = s.iter().step_by(len).skip(1).take_while(|tree| tree < &c).count();
    if  s.iter().step_by(len).skip(1).skip(c).next().is_none() {
        c
    } else {c+1}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("{}",process_one(&input));

    }

    #[test]
    fn part_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("{}",process_two(&input));

    }


    #[test]
    fn test_process_part_one() {
        let input = 
"30373
25512
65332
33549
35390";
        assert_eq!(process_one(input), "21");
    }

    #[test]
    fn test_process_part_two() {
        let input = 
"30373
25512
65332
33549
35390";
        assert_eq!(process_two(input), "8");
    }
}

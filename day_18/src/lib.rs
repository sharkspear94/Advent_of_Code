#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn count_sides(matrix: &Vec<Vec<Vec<bool>>>) -> usize {
    let mut sum = 0;
    for (i, x) in matrix.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            for (k, z) in y.iter().enumerate() {
                if *z {
                    sum += check_sides([i, j, k], matrix);
                }
            }
        }
    }
    sum
}

fn check_air_bubbles(matrix: &Vec<Vec<Vec<bool>>>) -> usize {
    let start = [0, 0, 0];
    let mut q = std::collections::VecDeque::from([start]);
    let mut visited = HashSet::from([start]);
    let mut sum = 0;
    while let Some(coord) = q.pop_front() {
        sum += 6 - check_sides(coord, matrix);
        let next = visit_sides(coord, matrix, &visited);
        next.iter().flat_map(|a| a).for_each(|c| {
            visited.insert(*c);
        });
        q.extend(next.into_iter().flat_map(|a| a))
    }

    sum
}

fn visit_sides(
    [x, y, z]: [usize; 3],
    matrix: &Vec<Vec<Vec<bool>>>,
    visited: &HashSet<[usize; 3]>,
) -> [Option<[usize; 3]>; 6] {
    let side1 = matrix
        .get(x + 1)
        .map(|a| a[y][z])
        .filter(|_| !visited.contains(&[x + 1, y, z]))
        .filter(|b| !b)
        .map(|_| [x + 1, y, z]);
    let side2 = x
        .checked_sub(1)
        .map(|a| matrix[a][y][z])
        .filter(|_| !visited.contains(&[x - 1, y, z]))
        .filter(|b| !b)
        .map(|_| [x - 1, y, z]);
    let side3 = matrix[x]
        .get(y + 1)
        .map(|a| a[z])
        .filter(|_| !visited.contains(&[x, y + 1, z]))
        .filter(|b| !b)
        .map(|_| [x, y + 1, z]);
    let side4 = y
        .checked_sub(1)
        .map(|a| matrix[x][a][z])
        .filter(|_| !visited.contains(&[x, y - 1, z]))
        .filter(|b| !b)
        .map(|_| [x, y - 1, z]);
    let side5 = matrix[x][y]
        .get(z + 1)
        .filter(|_| !visited.contains(&[x, y, z + 1]))
        .filter(|&b| !b)
        .map(|_| [x, y, z + 1]);
    let side6 = z
        .checked_sub(1)
        .map(|a| matrix[x][y][a])
        .filter(|_| !visited.contains(&[x, y, z - 1]))
        .filter(|b| !b)
        .map(|_| [x, y, z - 1]);

    [side1, side2, side3, side4, side5, side6]
}

fn check_sides([x, y, z]: [usize; 3], matrix: &Vec<Vec<Vec<bool>>>) -> usize {
    let side1 = matrix
        .get(x + 1)
        .map(|a| a[y][z])
        .map(|b| Into::into(!b))
        .unwrap_or(1);
    let side2 = x
        .checked_sub(1)
        .map(|a| matrix[a][y][z])
        .map(|b| Into::into(!b))
        .unwrap_or(1);
    let side3 = matrix[x]
        .get(y + 1)
        .map(|a| a[z])
        .map(|b| Into::into(!b))
        .unwrap_or(1);
    let side4 = y
        .checked_sub(1)
        .map(|a| matrix[x][a][z])
        .map(|b| Into::into(!b))
        .unwrap_or(1);
    let side5 = matrix[x][y].get(z + 1).map(|b| Into::into(!b)).unwrap_or(1);
    let side6 = z
        .checked_sub(1)
        .map(|a| matrix[x][y][a])
        .map(|b| Into::into(!b))
        .unwrap_or(1);
    side1 + side2 + side3 + side4 + side5 + side6
}

pub fn proccess_one(input: &str) -> usize {
    let max = input
        .lines()
        .flat_map(|coord| coord.split(',').flat_map(|len| len.parse::<usize>()))
        .max()
        .unwrap();
    let mut m = vec![vec![vec![false; max + 1]; max + 1]; max + 1];
    input
        .lines()
        .flat_map(|coord| coord.split(',').flat_map(|len| len.parse::<usize>()))
        .array_chunks()
        .for_each(|[x, y, z]| m[x][y][z] = true);
    let sum = count_sides(&m);
    sum
}
pub fn proccess_two(input: &str) -> usize {
    let max = input
        .lines()
        .flat_map(|coord| coord.split(',').flat_map(|len| len.parse::<usize>()))
        .max()
        .unwrap();
    let mut m = vec![vec![vec![false; max + 3]; max + 3]; max + 3];
    input
        .lines()
        .flat_map(|coord| coord.split(',').flat_map(|len| len.parse::<usize>()))
        .array_chunks()
        .for_each(|[x, y, z]| m[x + 1][y + 1][z + 1] = true);
    check_air_bubbles(&m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part one: {}", proccess_one(&input));
    }

    #[test]
    fn part_two() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part two: {}", proccess_two(&input));
    }

    #[test]
    fn test_part_one() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(proccess_one(input), 64);
    }

    #[test]
    fn test_part_two() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(proccess_two(input), 58);
    }

    #[test]
    fn test_part_two2() {
        let input = "1,1,1
2,1,1
3,1,1
4,1,1
5,1,1
6,1,1
1,2,1
2,2,1
3,2,1
4,2,1
5,2,1
6,2,1
1,3,1
2,3,1
3,3,1
4,3,1
5,3,1
6,3,1
1,1,2
2,1,2
3,1,2
4,1,2
5,1,2
6,1,2
1,2,2
6,2,2
1,3,2
2,3,2
3,3,2
4,3,2
5,3,2
6,3,2
1,1,3
2,1,3
3,1,3
4,1,3
5,1,3
6,1,3
1,2,3
2,2,3
3,2,3
4,2,3
5,2,3
6,2,3
1,3,3
2,3,3
3,3,3
4,3,3
5,3,3
6,3,3";
        assert_eq!(proccess_two(input), 90);
    }
}

use std::collections::{VecDeque, HashSet};

fn build_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| line.bytes()
            .map(|byte| if byte == b'S' {
                u8::MAX -1
            }else if byte == b'E' {
                b'z'+1
            } else {
                byte
            })
            .collect())
        .collect()
}

fn find_start(grid: &Vec<Vec<u8>>) -> (usize,usize) {
    let (i,_) = grid.iter().flatten().enumerate().find(|(_,&c)|c == u8::MAX -1).unwrap();
    (i/grid[0].len(),i%grid[0].len())
}

fn possible_steps(grid: &Vec<Vec<u8>>,(x,y): (usize,usize),visited: &HashSet<(usize,usize)>) -> [Option<(usize,usize)>;4] {
    let current = grid[x][y];
    let left = grid[x].get(y.checked_sub(1).unwrap_or(usize::MAX))
        .filter(|&&c| c <= current + 1)
        .map(|_| (x,y-1))
        .filter(|pos| !visited.contains(pos)); 
    let right = grid[x].get(y + 1)
        .filter(|&&c| c <= current + 1)
        .map(|_| (x,y+1))
        .filter(|pos| !visited.contains(pos)); 
    let up = grid.get(x.checked_sub(1).unwrap_or(usize::MAX))
        .map(|row|row[y])
        .filter(|&c|c <= current + 1)
        .map(|_| (x-1,y))
        .filter(|pos| !visited.contains(pos)); 
    let down = grid.get(x + 1)
        .map(|row|row[y])
        .filter(|&c|c <= current + 1)
        .map(|_| (x+1,y))
        .filter(|pos| !visited.contains(pos)); 
    [left,right,up,down]
}

fn bfs(grid: &Vec<Vec<u8>>,start:(usize,usize)) -> Option<usize> {
    let mut visited = HashSet::from([start]);
    let mut q = VecDeque::from([(start,0)]);
    while let Some(((x,y),len)) = q.pop_front() {
        if grid[x][y] == b'z'+1 {
            return Some(len);
        }
        let neighbors = possible_steps(&grid,(x,y),&visited);
        neighbors.into_iter()
            .flatten()
            .for_each(|pos| {visited.insert(pos);q.push_back((pos,len+1))});
    }
    None
}

pub fn proccess_one(input: &str) -> usize {
    let mut grid = build_grid(input);
    let start = find_start(&mut grid);

    bfs(&grid,start).unwrap()
}

fn start_positions(grid: &Vec<Vec<u8>>) -> Vec<(usize,usize)>{
    grid.iter()
        .flatten()
        .enumerate()
        .filter(|(_,&byte)| byte == b'a' || byte == u8::MAX-1)
        .map(|(i,_)|(i/grid[0].len(),i%grid[0].len()))
        .collect()
}

pub fn proccess_two(input: &str) -> usize {
    let grid = build_grid(input);
    start_positions(&grid).iter()
        .filter_map(|&start| bfs(&grid, start))
        .min().unwrap()
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn part_one() {
        let now = Instant::now();
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part one: {}, time: {}",proccess_one(&input), now.elapsed().as_millis());
    }

    #[test]
    fn part_two() {
        let now = Instant::now();
        let input = std::fs::read_to_string("./input.txt").unwrap();
        println!("Result part two: {}, time: {}",proccess_two(&input), now.elapsed().as_millis())
    }

    #[test]
    fn test_part_one() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(proccess_one(input), 31);
    }

    #[test]
    fn test_part_two() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(proccess_two(input), 29);
    }
}

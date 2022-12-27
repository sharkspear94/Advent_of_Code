use std::{
    collections::HashSet,
    fmt::{write, Display},
};

#[derive(Debug)]
struct Field {
    field: Vec<Vec<FieldType>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.field {
            for t in line {
                match t {
                    FieldType::Wall => write!(f, "#,"),
                    FieldType::Blizzards(a) if a.iter().all(|a| a.is_none()) => write!(f, ".,"),
                    FieldType::Blizzards([left, right, up, down]) => write!(f, "B,"),
                };
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

impl Field {
    fn tick(&mut self) {
        let mut new_field = Vec::with_capacity(self.field.len());
        new_field.extend((0..self.field.len()).map(|_| Vec::with_capacity(self.width)));
        new_field
            .first_mut()
            .map(|a| *a = self.field.first().unwrap().clone());
        for y in 1..self.field.len() - 1 {
            new_field[y].push(FieldType::Wall);
            for x in 1..self.width - 1 {
                new_field[y].push(FieldType::Blizzards(self.find((x, y))));
            }
            new_field[y].push(FieldType::Wall);
        }
        new_field
            .last_mut()
            .map(|a| *a = self.field.last().unwrap().clone());
        self.field = new_field
    }

    fn find(&self, (x, y): (usize, usize)) -> [Option<Blizzard>; 4] {
        let left = if x == self.width.saturating_sub(2) {
            match self.field[y][1] {
                FieldType::Wall => None,
                FieldType::Blizzards(a) => a
                    .iter()
                    .flatten()
                    .find(|a| matches!(a, Blizzard::Left))
                    .copied(),
            }
        } else {
            match self.field[y][x + 1] {
                FieldType::Wall => None,
                FieldType::Blizzards(a) => a
                    .iter()
                    .flatten()
                    .find(|a| matches!(a, Blizzard::Left))
                    .copied(),
            }
        };
        let right = if x == 1 {
            match self.field[y][self.width.saturating_sub(2)] {
                FieldType::Wall => None,
                FieldType::Blizzards(a) => a
                    .iter()
                    .flatten()
                    .find(|a| matches!(a, Blizzard::Right))
                    .copied(),
            }
        } else {
            match self.field[y][x - 1] {
                FieldType::Wall => None,
                FieldType::Blizzards(a) => a
                    .iter()
                    .flatten()
                    .find(|a| matches!(a, Blizzard::Right))
                    .copied(),
            }
        };
        let up = if y == self.field.len().saturating_sub(2) {
            match self.field[1][x] {
                FieldType::Wall => None,
                FieldType::Blizzards(a) => a
                    .iter()
                    .flatten()
                    .find(|a| matches!(a, Blizzard::Up))
                    .copied(),
            }
        } else {
            match self.field[y + 1][x] {
                FieldType::Wall => None,
                FieldType::Blizzards(a) => a
                    .iter()
                    .flatten()
                    .find(|a| matches!(a, Blizzard::Up))
                    .copied(),
            }
        };

        let down = if y == 1 {
            match self.field[self.field.len().saturating_sub(2)][x] {
                FieldType::Wall => None,
                FieldType::Blizzards(a) => a
                    .iter()
                    .flatten()
                    .find(|a| matches!(a, Blizzard::Down))
                    .copied(),
            }
        } else {
            match self.field[y - 1][x] {
                FieldType::Wall => None,
                FieldType::Blizzards(a) => a
                    .iter()
                    .flatten()
                    .find(|a| matches!(a, Blizzard::Down))
                    .copied(),
            }
        };
        [left, right, up, down]
    }
    fn new_pos(&self, (x, y): (usize, usize)) -> [Option<(usize, usize)>; 5] {
        let left = match self.field[y][x - 1] {
            FieldType::Blizzards(a) if a.iter().all(|f| f.is_none()) => Some((x - 1, y)),
            _ => None,
        };
        let right = match self.field[y][x + 1] {
            FieldType::Blizzards(a) if a.iter().all(|f| f.is_none()) => Some((x + 1, y)),
            _ => None,
        };
        let down = self.field.get(y + 1).and_then(|line| match line[x] {
            FieldType::Blizzards(a) if a.iter().all(|f| f.is_none()) => Some((x, y + 1)),
            _ => None,
        });
        let up = y
            .checked_sub(1)
            .and_then(|checked_y| match self.field[checked_y][x] {
                FieldType::Blizzards(a) if a.iter().all(|f| f.is_none()) => Some((x, checked_y)),
                _ => None,
            });
        let mid = match self.field[y][x] {
            FieldType::Blizzards(a) if a.iter().all(|f| f.is_none()) => Some((x, y)),
            _ => None,
        };
        [left, right, up, down, mid]
    }
}

#[derive(Debug, Clone, Copy)]
enum FieldType {
    Wall,
    Blizzards([Option<Blizzard>; 4]),
}

#[derive(Debug, Clone, Copy)]
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Field {
    let field = input
        .lines()
        .map(|line| {
            line.chars().fold(Vec::new(), |mut v, c| {
                v.push(match c {
                    '#' => FieldType::Wall,
                    '.' => FieldType::Blizzards([None; 4]),
                    '>' => FieldType::Blizzards([Some(Blizzard::Right), None, None, None]),
                    '<' => FieldType::Blizzards([Some(Blizzard::Left), None, None, None]),
                    '^' => FieldType::Blizzards([Some(Blizzard::Up), None, None, None]),
                    'v' => FieldType::Blizzards([Some(Blizzard::Down), None, None, None]),
                    rest => panic!("{rest} found, not a valid character"),
                });
                v
            })
        })
        .collect::<Vec<_>>();
    let start = (
        field
            .first()
            .and_then(|f| {
                f.iter().position(
                    |t| matches!(t, FieldType::Blizzards(a) if a.iter().all(|a| a.is_none())),
                )
            })
            .unwrap(),
        0,
    );
    let end = (
        field
            .last()
            .and_then(|f| {
                f.iter().position(
                    |t| matches!(t, FieldType::Blizzards(a) if a.iter().all(|a| a.is_none())),
                )
            })
            .unwrap(),
        field.len() - 1,
    );
    let width = field[0].len();
    Field {
        field,
        start,
        end,
        width,
    }
}

fn find_path(mut field: Field) -> usize {
    let mut positions = HashSet::from([field.start]);

    let mut minutes = 0;
    loop {
        positions = positions.into_iter().fold(HashSet::new(), |mut set, pos| {
            set.extend(field.new_pos(pos).into_iter().flatten());
            set
        });
        if positions.contains(&field.end) {
            return minutes;
        }
        field.tick();
        minutes += 1;
    }
}

fn find_paths(mut field: Field) -> usize {
    let mut positions = HashSet::from([field.start]);

    let mut minutes = 0;
    let first = loop {
        positions = positions.into_iter().fold(HashSet::new(), |mut set, pos| {
            set.extend(field.new_pos(pos).into_iter().flatten());
            set
        });
        if positions.contains(&field.end) {
            break minutes;
        }
        field.tick();
        minutes += 1;
    };
    let mut minutes = 0;
    let mut positions = HashSet::from([field.end]);
    let back = loop {
        positions = positions.into_iter().fold(HashSet::new(), |mut set, pos| {
            set.extend(field.new_pos(pos).into_iter().flatten());
            set
        });
        if positions.contains(&field.start) {
            break minutes;
        }
        field.tick();
        minutes += 1;
    };
    let mut positions = HashSet::from([field.start]);
    let mut minutes = 0;
    let back_again = loop {
        positions = positions.into_iter().fold(HashSet::new(), |mut set, pos| {
            set.extend(field.new_pos(pos).into_iter().flatten());
            set
        });
        if positions.contains(&field.end) {
            break minutes;
        }
        field.tick();
        minutes += 1;
    };
    first + back + back_again
}

pub fn process1(input: &str) -> usize {
    let field = parse(input);
    find_path(field)
}

pub fn process2(input: &str) -> usize {
    let field = parse(input);
    find_paths(field)
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
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        let result = process1(input);
        assert_eq!(result, 18)
    }

    #[test]
    
    fn process_two_test() {
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        let result = process2(input);
        assert_eq!(result, 54)
    }
}

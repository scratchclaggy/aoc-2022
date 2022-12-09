use std::{collections::HashSet, str::FromStr};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Not a valid direction".to_string()),
        }
    }
}

fn follow_head(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let x_delta = head.0 - tail.0;
    let y_delta = head.1 - tail.1;

    if i32::abs(x_delta) < 2 && i32::abs(y_delta) < 2 {
        return tail.clone();
    }

    let x_delta = i32::clamp(x_delta, -1, 1);
    let y_delta = i32::clamp(y_delta, -1, 1);

    (tail.0 + x_delta, tail.1 + y_delta)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    input.lines().for_each(|ln| {
        let mut ln = ln.split_whitespace();
        let direction: Direction = ln.next().unwrap().parse().unwrap();
        let distance: u32 = ln.next().unwrap().parse().unwrap();

        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for _ in 0..distance {
            head.0 += dx;
            head.1 += dy;

            tail = follow_head(head, tail);
            visited.insert(tail);
        }
    });

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = vec![(0, 0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    input.lines().for_each(|ln| {
        let mut ln = ln.split_whitespace();
        let direction: Direction = ln.next().unwrap().parse().unwrap();
        let distance: u32 = ln.next().unwrap().parse().unwrap();

        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for _ in 0..distance {
            rope[0].0 += dx;
            rope[0].1 += dy;

            for i in 1..rope.len() {
                rope[i] = follow_head(rope[i - 1], rope[i])
            }

            visited.insert(*rope.last().unwrap());
        }
    });

    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part_two(&input), Some(36));
    }
}

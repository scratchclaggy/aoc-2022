pub fn part_one(input: &str) -> Option<u32> {
    let mut forrest = vec![vec![]; input.lines().next().unwrap().len()];
    input.lines().enumerate().for_each(|(i, ln)| {
        ln.chars()
            .for_each(|tree| forrest[i].push((false, tree.to_digit(10).unwrap() as u8)))
    });

    let height = forrest.len();
    let width = forrest.first().unwrap().len();

    // North
    for x in 0..width {
        let mut max_height = 0;
        for y in 0..height {
            if y == 0 {
                forrest[y][x].0 = true;
                max_height = forrest[y][x].1;
                continue;
            }

            let current = &mut forrest[y][x];

            if max_height < current.1 {
                current.0 = true;
                max_height = current.1;
            }
        }
    }

    // West
    for y in 0..height {
        let mut max_height = 0;
        for x in 0..width {
            if x == 0 {
                forrest[y][x].0 = true;
                max_height = forrest[y][x].1;
                continue;
            }

            let current = &mut forrest[y][x];

            if max_height < current.1 {
                current.0 = true;
                max_height = current.1;
            }
        }
    }

    // South
    for x in 0..width {
        let mut max_height = 0;
        for y in (0..height).rev() {
            if y == height - 1 {
                forrest[y][x].0 = true;
                max_height = forrest[y][x].1;
                continue;
            }

            let current = &mut forrest[y][x];

            if max_height < current.1 {
                current.0 = true;
                max_height = current.1;
            }
        }
    }

    // East
    for y in 0..height {
        let mut max_height = 0;
        for x in (0..width).rev() {
            if x == width - 1 {
                forrest[y][x].0 = true;
                max_height = forrest[y][x].1;
                continue;
            }

            let current = &mut forrest[y][x];

            if max_height < current.1 {
                current.0 = true;
                max_height = current.1;
            }
        }
    }

    Some(
        forrest
            .into_iter()
            .flatten()
            .filter_map(|(visible, _)| visible.then_some(true))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut forrest = vec![vec![]; input.lines().next().unwrap().len()];
    input.lines().enumerate().for_each(|(i, ln)| {
        ln.chars()
            .for_each(|tree| forrest[i].push(tree.to_digit(10).unwrap() as u8))
    });

    let height = forrest.len();
    let width = forrest.first().unwrap().len();

    let mut scores = vec![];
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);
            let treehouse_height = forrest[y][x];

            // Up
            for i in (0..y).rev() {
                up += 1;
                if forrest[i][x] >= treehouse_height {
                    break;
                }
            }

            // Down
            for i in y + 1..height {
                down += 1;
                if forrest[i][x] >= treehouse_height {
                    break;
                }
            }

            // Left
            for i in (0..x).rev() {
                left += 1;
                if forrest[y][i] >= treehouse_height {
                    break;
                }
            }

            // Right
            for i in x + 1..width {
                right += 1;
                if forrest[y][i] >= treehouse_height {
                    break;
                }
            }

            scores.push(up * down * left * right);
        }
    }

    Some( *scores.iter().max().unwrap() )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}

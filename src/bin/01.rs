use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|inventory| {
                inventory
                    .lines()
                    .fold(0, |acc, item| acc + item.parse::<u32>().unwrap())
            })
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|inventory| {
                inventory
                    .split("\n")
                    .fold(0, |acc, item| acc + item.parse::<u32>().unwrap_or(0))
            })
            .collect::<BinaryHeap<u32>>()
            .into_iter()
            .take(3)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}

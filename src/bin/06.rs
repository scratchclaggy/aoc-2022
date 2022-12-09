use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let window_size = 4;

    Some(
        (input
            .chars()
            .collect::<Vec<_>>()
            .windows(window_size)
            .enumerate()
            .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == window_size)
            .unwrap()
            .0
            + window_size) as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let window_size = 14;

    Some(
        (input
            .chars()
            .collect::<Vec<_>>()
            .windows(window_size)
            .enumerate()
            .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == window_size)
            .unwrap()
            .0
            + window_size) as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}

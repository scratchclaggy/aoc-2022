use std::collections::HashSet;

const UPPERCASE: u32 = 'A' as u32;
const LOWERCASE: u32 = 'a' as u32;

fn char_as_num(c: char) -> u32 {
    let c = c as u32;
    if c >= LOWERCASE {
        c - LOWERCASE + 1
    } else {
        c - UPPERCASE + 27
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|ln| {
                let (left, right) = ln.split_at(ln.len() / 2);
                let set = left.chars().collect::<HashSet<char>>();
                let duplicate = right
                    .chars()
                    .find(|c| set.contains(c))
                    .expect(&format!("{:?} <-> {:?}", left, right));

                char_as_num(duplicate)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|ln| ln.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .chunks_exact_mut(3)
            .map(|chunk| {
                let duplicate = chunk[1..]
                    .iter()
                    .fold(chunk[0].clone(), |mut acc, set| {
                        acc.retain(|item| set.contains(item));
                        acc
                    })
                    .drain()
                    .next()
                    .unwrap();
                char_as_num(duplicate)
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}

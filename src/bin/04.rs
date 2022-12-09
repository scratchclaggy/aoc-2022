pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|ln| {
                ln.split(",").flat_map(|range| {
                    range
                        .split("-")
                        .map(|x| x.parse::<u32>().expect("Error parsing num"))
                })
            })
            .collect::<Vec<u32>>()
            .chunks_exact(4)
            .filter(|chunk| {
                if let [a, b, y, z] = chunk {
                    (a <= y && z <= b) || (y <= a && b <= z)
                } else {
                    unreachable!()
                }
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|ln| {
                ln.split(",").flat_map(|range| {
                    range
                        .split("-")
                        .map(|x| x.parse::<u32>().expect("Error parsing num"))
                })
            })
            .collect::<Vec<u32>>()
            .chunks_exact(4)
            .filter(|chunk| {
                if let [a, b, y, z] = chunk {
                    (a <= y && z <= b)
                        || (y <= a && b <= z)
                        || (a <= y && y <= b)
                        || (y <= a && a <= z)
                } else {
                    unreachable!()
                }
            })
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut pc = 0;
    let mut rx = 1;
    let mut acc: Vec<i32> = vec![];

    input.lines().for_each(|ln| {
        pc += 1;

        if pc % 40 == 20 {
            acc.push(pc * rx);
        }

        if ln.starts_with("addx") {
            pc += 1;
            if pc % 40 == 20 {
                acc.push(pc * rx);
            }

            rx += ln
                .split_whitespace()
                .into_iter()
                .skip(1)
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    });

    Some(acc.iter().sum())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut pc = 0;
    let mut rx = 1;
    let mut output = String::new();

    input.lines().for_each(|ln| {
        if (rx - 1..=rx + 1).contains(&(pc % 40)) {
            output.push('#');
        } else {
            output.push('.');
        }
        pc += 1;
        if pc % 40 == 0 {
            output.push('\n');
        }

        if ln.starts_with("addx") {
            if (rx - 1..=rx + 1).contains(&(pc % 40)) {
                output.push('#');
            } else {
                output.push('.');
            }
            pc += 1;
            if pc % 40 == 0 {
                output.push('\n');
            }

            rx += ln
                .split_whitespace()
                .into_iter()
                .skip(1)
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    });

    Some(output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
                    .to_string()
            )
        );
    }
}

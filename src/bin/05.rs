pub fn part_one(input: &str) -> Option<String> {
    let (crates, instructions): (Vec<_>, Vec<_>) =
        input.lines().partition(|ln| ln.trim().starts_with("["));

    let crates = crates.iter().map(|ln| {
        ln.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| {
                let c = chunk.iter().find(|c| c.is_alphabetic());
                match c {
                    Some(c) => c.clone(),
                    None => '.',
                }
            })
            .collect::<Vec<_>>()
    });

    let len = crates.clone().next().unwrap().len();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; len];

    crates.rev().for_each(|ln| {
        ln.iter().enumerate().for_each(|(i, c)| match c != &'.' {
            true => stacks[i].push(c.clone()),
            false => {}
        })
    });

    instructions
        .iter()
        .skip(2)
        .map(|ln| {
            let mut ln = ln
                .split_whitespace()
                .filter_map(|w| w.parse::<usize>().ok());
            let qty = ln.next().unwrap();
            let from = ln.next().unwrap() - 1;
            let to = ln.next().unwrap() - 1;
            (qty, from, to)
        })
        .for_each(|(qty, from, to)| {
            for _ in 0..qty {
                let from = stacks[from].pop().unwrap();
                stacks[to].push(from);
            }
        });

    Some(
        stacks
            .iter_mut()
            .map(|stack| stack.pop().unwrap())
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (crates, instructions): (Vec<_>, Vec<_>) =
        input.lines().partition(|ln| ln.trim().starts_with("["));

    let crates = crates.iter().map(|ln| {
        ln.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| {
                let c = chunk.iter().find(|c| c.is_alphabetic());
                match c {
                    Some(c) => c.clone(),
                    None => '.',
                }
            })
            .collect::<Vec<_>>()
    });

    let len = crates.clone().next().unwrap().len();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; len];

    crates.rev().for_each(|ln| {
        ln.iter().enumerate().for_each(|(i, c)| match c != &'.' {
            true => stacks[i].push(c.clone()),
            false => {}
        })
    });

    instructions
        .iter()
        .skip(2)
        .map(|ln| {
            let mut ln = ln
                .split_whitespace()
                .filter_map(|w| w.parse::<usize>().ok());
            let qty = ln.next().unwrap();
            let from = ln.next().unwrap() - 1;
            let to = ln.next().unwrap() - 1;
            (qty, from, to)
        })
        .for_each(|(qty, from, to)| {
            let mut temp = vec![];
            for _ in 0..qty {
                let from = stacks[from].pop().unwrap();
                temp.push(from);
            }
            stacks[to].extend(temp.iter().rev());
        });

    Some(
        stacks
            .iter_mut()
            .map(|stack| stack.pop().unwrap())
            .collect::<String>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}

use std::str::FromStr;

#[derive(PartialEq, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Invalid hand".to_string()),
        }
    }
}

#[derive(PartialEq)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Lose),
            "B" | "Y" => Ok(Self::Draw),
            "C" | "Z" => Ok(Self::Win),
            _ => Err("Invalid outcome".to_string()),
        }
    }
}

fn rock_paper_scissors(us: &Move, them: &Move) -> Outcome {
    if us == them {
        Outcome::Draw
    } else {
        match (us, them) {
            (Move::Rock, Move::Paper) => Outcome::Lose,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Scissors) => Outcome::Lose,
            (Move::Scissors, Move::Rock) => Outcome::Lose,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

fn correct_move(outcome: &Outcome, them: &Move) -> Move {
    if outcome == &Outcome::Draw {
        them.clone()
    } else {
        match (outcome, them) {
            (Outcome::Lose, Move::Rock) => Move::Scissors,
            (Outcome::Lose, Move::Paper) => Move::Rock,
            (Outcome::Lose, Move::Scissors) => Move::Paper,
            (Outcome::Win, Move::Rock) => Move::Paper,
            (Outcome::Win, Move::Paper) => Move::Scissors,
            (Outcome::Win, Move::Scissors) => Move::Rock,
            _ => unreachable!(),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|game| {
                let mut game = game.split(' ').map(|s| s.parse().unwrap());
                let them = game.next().unwrap();
                let us = game.next().unwrap();

                rock_paper_scissors(&us, &them) as u32 + us as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|game| {
                let mut game = game.split(' ');
                let them: Move = game.next().unwrap().parse().unwrap();
                let outcome: Outcome = game.next().unwrap().parse().unwrap();

                correct_move(&outcome, &them) as u32 + outcome as u32
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}

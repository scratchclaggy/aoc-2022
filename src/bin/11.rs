use std::collections::{BinaryHeap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

#[derive(Debug)]
enum Val {
    Prev,
    Num(u64),
}

#[derive(Debug)]
enum Op {
    Sum(Val, Val),
    Prod(Val, Val),
}

#[derive(Debug)]
struct Test {
    divisible_by: u64,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    inspections: u64,
    op: Op,
    test: Test,
}

fn val(input: &str) -> IResult<&str, Val> {
    alt((
        tag("old").map(|_| Val::Prev),
        complete::u64.map(|x| Val::Num(x)),
    ))(input)
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, _) = multispace1(input)?;
    let (input, divisible_by) = preceded(tag("Test: divisible by "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_monkey) = preceded(tag("If true: throw to monkey "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_monkey) = preceded(tag("If false: throw to monkey "), complete::u64)(input)?;

    Ok((
        input,
        Test {
            divisible_by,
            true_monkey: true_monkey as usize,
            false_monkey: false_monkey as usize,
        },
    ))
}

fn op(input: &str) -> IResult<&str, Op> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, lhs) = val(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, rhs) = val(input)?;

    Ok((
        input,
        match operator {
            "*" => Op::Prod(lhs, rhs),
            "+" => Op::Sum(lhs, rhs),
            _ => panic!(),
        },
    ))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), complete::u64),
    )(input)?;
    let (input, op) = op(input)?;
    let (input, test) = test(input)?;
    Ok((
        input,
        Monkey {
            items: items.into(),
            inspections: 0,
            op,
            test,
        },
    ))
}

impl Monkey {
    fn inspect(&mut self, is_part_one: bool, lcm: u64) -> Option<u64> {
        if let Some(mut item) = self.items.pop_front() {
            self.inspections += 1;

            item = match &self.op {
                Op::Sum(lhs, rhs) => {
                    let lhs = match lhs {
                        Val::Prev => item,
                        Val::Num(x) => *x,
                    };
                    let rhs = match rhs {
                        Val::Prev => item,
                        Val::Num(x) => *x,
                    };
                    lhs + rhs
                }
                Op::Prod(lhs, rhs) => {
                    let lhs = match lhs {
                        Val::Prev => item,
                        Val::Num(x) => *x,
                    };
                    let rhs = match rhs {
                        Val::Prev => item,
                        Val::Num(x) => *x,
                    };
                    lhs * rhs
                }
            };

            if is_part_one {
                item /= 3;
            } else {
                item %= lcm;
            }

            Some(item)
        } else {
            None
        }
    }

    fn test(&self, item: u64) -> usize {
        if item % self.test.divisible_by == 0 {
            self.test.true_monkey
        } else {
            self.test.false_monkey
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, mut monkeys) = separated_list1(multispace1, monkey)(input).unwrap();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].inspect(true, 0) {
                let receiver = monkeys[i].test(item);
                monkeys[receiver].items.push_back(item);
            }
        }
    }

    Some(
        monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<BinaryHeap<_>>()
            .iter()
            .take(2)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut monkeys) = separated_list1(multispace1, monkey)(input).unwrap();

    let lcm = monkeys.iter().map(|m| m.test.divisible_by).product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].inspect(false, lcm) {
                let receiver = monkeys[i].test(item);
                monkeys[receiver].items.push_back(item);
            }
        }
    }

    Some(
        monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<BinaryHeap<_>>()
            .iter()
            .take(2)
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}

use itertools::Itertools;
use std::cmp::PartialEq;

advent_of_code::solution!(7);

#[derive(Debug)]
pub struct Input {
    result: u64,
    variables: Vec<u64>,
}

pub fn parse(input: &str) -> Vec<Input> {
    let lines = input.lines();

    lines
        .map(|line| Input {
            result: line.split(':').nth(0).unwrap().parse::<u64>().ok().unwrap(),
            variables: line
                .split(':')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|str| str.parse::<u64>().ok().unwrap())
                .collect_vec(),
        })
        .collect_vec()
}

struct BooleanPermutations {
    n: usize,
    current: u32,
    total: u32,
}

impl BooleanPermutations {
    fn new(n: usize) -> Self {
        let total = 1 << n; // 2^n total permutations
        BooleanPermutations {
            n,
            current: 0,
            total,
        }
    }
}

impl Iterator for BooleanPermutations {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.total {
            return None; // No more permutations
        }

        let result = (0..self.n)
            .map(|bit| (self.current & (1 << bit)) != 0) // Check if bit is set
            .collect();

        self.current += 1; // Move to the next number
        Some(result)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = parse(input);

    let answer = parsed
        .iter()
        .filter_map(|input| {
            let mut permutations = BooleanPermutations::new(input.variables.len());

            permutations.find_map(|permutation| {
                let mut result = input.variables[0];
                for (index, variable) in input.variables.iter().enumerate().skip(1) {
                    if permutation[index] {
                        result *= variable
                    } else {
                        result += variable
                    }
                }

                if result == input.result {
                    Some(result)
                } else {
                    None
                }
            })
        })
        .sum();

    Some(answer)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Multiply,
    Add,
    Concat,
}

struct OperationPermutations {
    n: usize,
    current: u64,
    total: u64,
}

impl OperationPermutations {
    fn new(n: usize) -> Self {
        let total = 3_u64.pow(n as u32); // 3^n total permutations (3 variants)
        OperationPermutations {
            n,
            current: 0,
            total,
        }
    }
}

impl Iterator for OperationPermutations {
    type Item = Vec<Operation>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.total {
            return None; // No more permutations
        }

        let mut result = Vec::with_capacity(self.n);
        let mut num = self.current;

        for _ in 0..self.n {
            let op = match num % 3 {
                0 => Operation::Multiply,
                1 => Operation::Add,
                2 => Operation::Concat,
                _ => unreachable!(), // Only 3 values are possible
            };
            result.push(op);
            num /= 3;
        }

        self.current += 1; // Move to the next number
        Some(result)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = parse(input);

    let answer = parsed
        .iter()
        .filter_map(|input| {
            let mut permutations = OperationPermutations::new(input.variables.len());

            permutations.find_map(|permutation| {
                let mut result = input.variables[0];
                for (index, variable) in input.variables.iter().enumerate().skip(1) {
                    if permutation[index] == Operation::Add {
                        result += variable
                    } else if permutation[index] == Operation::Multiply {
                        result *= variable
                    } else if permutation[index] == Operation::Concat {
                        let number_of_digits = match variable {
                            0 => 1,
                            _ => (*variable as f64).log10().floor() as u32 + 1,
                        };

                        let multiplier = 10_u64.pow(number_of_digits);

                        result = result * multiplier + variable;
                    }
                }

                if result == input.result {
                    Some(result)
                } else {
                    None
                }
            })
        })
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

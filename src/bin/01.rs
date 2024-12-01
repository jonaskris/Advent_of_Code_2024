advent_of_code::solution!(1);
use itertools::Itertools;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32)>()
                .unwrap()
        })
        .multiunzip::<(Vec<u32>, Vec<u32>)>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parsed_input = parse(input);

    parsed_input.0.sort();
    parsed_input.1.sort();

    let answer = std::iter::zip(parsed_input.0, parsed_input.1)
        .map(|(x, y)| x.abs_diff(y))
        .sum::<u32>();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse(input);

    let answer = parsed_input
        .0
        .iter()
        .map(|i| i * parsed_input.1.iter().filter(|j| i.eq(j)).count() as u32)
        .sum::<u32>();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

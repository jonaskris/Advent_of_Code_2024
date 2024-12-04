advent_of_code::solution!(3);

use itertools::Either;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let multiplication_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let multiplications: Vec<(u32, u32)> = multiplication_re
        .captures_iter(input)
        .filter_map(|cap| {
            Some((
                cap.get(1)?.as_str().parse::<u32>().ok()?,
                cap.get(2)?.as_str().parse::<u32>().ok()?,
            ))
        })
        .collect();

    let answer = multiplications.iter().map(|(a, b)| a * b).sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let command_re = Regex::new(
        r"(?<multiplication>mul\((\d+),(\d+)\))|(?<dokeyword>do\(\))|(?<dontkeyword>don't\(\))",
    )
    .unwrap();
    let multiplication_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let commands: Vec<Either<(u32, u32), &str>> = command_re
        .captures_iter(input)
        .filter_map(|cap| {
            if let Some(multiplication) = cap.name("multiplication") {
                let nums: Option<(u32, u32)> = multiplication_re
                    .captures(multiplication.as_str())
                    .and_then(|cap| {
                        Some((
                            cap.get(1)?.as_str().parse::<u32>().ok()?,
                            cap.get(2)?.as_str().parse::<u32>().ok()?,
                        ))
                    });

                nums.map(Either::Left)
            } else if let Some(dokeyword) = cap.name("dokeyword") {
                Some(Either::Right(dokeyword.as_str()))
            } else if let Some(dontkeyword) = cap.name("dontkeyword") {
                Some(Either::Right(dontkeyword.as_str()))
            } else {
                None
            }
        })
        .collect();

    let mut currently_dont = false;
    let answer = commands.iter().fold(0, |acc, command| match command {
        Either::Left((a, b)) if !currently_dont => acc + (a * b),
        Either::Right("do()") => {
            currently_dont = false;
            acc
        }
        Either::Right("don't()") => {
            currently_dont = true;
            acc
        }
        _ => acc,
    });

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}

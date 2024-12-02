advent_of_code::solution!(2);
use itertools::Itertools;

type Report = Vec<u32>;
type Input = Vec<Report>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

pub fn check_safety(report: &Report) -> bool {
    let increasing_or_decreasing_satisfied = report
        .windows(2)
        .flat_map(<&[u32; 2]>::try_from)
        .all(|[left, right]| left <= right)
        || report
            .windows(2)
            .flat_map(<&[u32; 2]>::try_from)
            .all(|[left, right]| left >= right);

    if !increasing_or_decreasing_satisfied {
        return false;
    };

    report
        .windows(2)
        .flat_map(<&[u32; 2]>::try_from)
        .all(|[left, right]| {
            let diff = left.abs_diff(*right);
            diff >= 1 && diff <= 3
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse(input);

    let answer = parsed_input
        .iter()
        .filter(|report| check_safety(&report))
        .count();

    Some(answer as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse(input);

    let answer = parsed_input
        .iter()
        .filter(|report| {
            if check_safety(&report) {
                true
            } else {
                for i in 0..report.len() {
                    let mut new_report = (*report).clone();
                    new_report.remove(i);
                    return if check_safety(&new_report) {
                        true
                    } else {
                        continue;
                    };
                }
                return false;
            }
        })
        .count();

    Some(answer as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

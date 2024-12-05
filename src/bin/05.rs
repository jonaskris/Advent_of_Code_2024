use itertools::Itertools;

advent_of_code::solution!(5);

type Rule = (u32, u32);
type Update = Vec<u32>;

pub struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

pub fn parse(input: &str) -> Input {
    Input {
        rules: input
            .lines()
            .take_while(|line| *line != "")
            .flat_map(|line| {
                line.split('|')
                    .map(|strnum| strnum.parse::<u32>().ok().unwrap())
                    .collect_tuple::<(u32, u32)>()
                    .into_iter()
                    .collect_vec()
            })
            .collect_vec(),
        updates: input
            .lines()
            .rev()
            .take_while_ref(|line| *line != "")
            .map(|line| {
                line.split(',')
                    .map(|strnum| strnum.parse::<u32>().ok().unwrap())
                    .collect_vec()
            })
            .collect_vec()
            .into_iter()
            .rev()
            .collect_vec(),
    }
}

pub fn find_broken_rule_in_update(update: &Update, rules: &Vec<Rule>) -> Option<Rule> {
    let mut potential_rule_breaks: Vec<&Rule> = vec![];

    for page in update.iter() {
        for potential_rule_break in &potential_rule_breaks {
            if potential_rule_break.0 == *page {
                return Some(**potential_rule_break);
            }
        }

        potential_rule_breaks
            .append(&mut rules.iter().filter(|rule| rule.1 == *page).collect_vec());
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let manual = parse(input);

    let valid_updates = manual
        .updates
        .into_iter()
        .filter_map(|update| {
            if find_broken_rule_in_update(&update, &manual.rules).is_none() {
                Some(update)
            } else {
                None
            }
        })
        .collect_vec();

    let answer = valid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let manual = parse(input);

    let invalid_updates = manual
        .updates
        .into_iter()
        .filter_map(|update| {
            if find_broken_rule_in_update(&update, &manual.rules).is_some() {
                Some(update)
            } else {
                None
            }
        })
        .collect_vec();

    let fixed_invalid_updates = invalid_updates
        .into_iter()
        .map(|mut update| {
            let mut is_valid = false;

            while !is_valid {
                if let Some(broken_rule) = find_broken_rule_in_update(&update, &manual.rules) {
                    let (left, right) = broken_rule;

                    let left_index = update
                        .iter()
                        .enumerate()
                        .find_map(|(index, page)| if *page == left { Some(index) } else { None })
                        .unwrap();
                    let right_index = update
                        .iter()
                        .enumerate()
                        .find_map(|(index, page)| if *page == right { Some(index) } else { None })
                        .unwrap();

                    update.swap(left_index, right_index);
                } else {
                    is_valid = true;
                }
            }

            update
        })
        .collect_vec();

    let answer = fixed_invalid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

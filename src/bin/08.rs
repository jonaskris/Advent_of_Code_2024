use itertools::Itertools;

advent_of_code::solution!(8);

pub struct Input {
    map: Vec<Vec<char>>,
    antennas_by_frequency: Vec<(char, Vec<Vector>)>,
}

#[derive(Debug)]
pub struct Vector {
    x: isize,
    y: isize,
}

pub fn parse(input: &str) -> Input {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut antennas_by_frequency: Vec<(char, Vec<Vector>)> = vec![];

    map.iter().enumerate().for_each(|(y, chars)| {
        chars
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != '.')
            .for_each(|(x, new_antenna)| {
                let existing_antenna_collection = antennas_by_frequency
                    .iter_mut()
                    .find(|(frequency, _)| frequency == new_antenna);

                if let Some(existing_antenna_collection) = existing_antenna_collection {
                    existing_antenna_collection.1.push(Vector {
                        x: x as isize,
                        y: y as isize,
                    });
                } else {
                    antennas_by_frequency.push((
                        *new_antenna,
                        vec![Vector {
                            x: x as isize,
                            y: y as isize,
                        }],
                    ))
                };
            })
    });

    Input {
        map,
        antennas_by_frequency,
    }
}

fn is_within_bounds<T>(grid: &Vec<Vec<T>>, x: isize, y: isize) -> bool {
    y >= 0 && y < (grid.len() as isize) && x >= 0 && x < (grid[y as usize].len() as isize)
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse(input);

    let mut antinode_array: Vec<Vec<bool>> = parsed
        .map
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();

    parsed
        .antennas_by_frequency
        .iter()
        .for_each(|(_, antennas)| {
            for (first_index, first_antenna) in antennas.iter().enumerate() {
                for second_antenna in antennas.iter().skip(first_index + 1) {
                    let first_antinode = Vector {
                        x: first_antenna.x + ((second_antenna.x - first_antenna.x) * 2),
                        y: first_antenna.y + ((second_antenna.y - first_antenna.y) * 2),
                    };

                    let second_antinode = Vector {
                        x: second_antenna.x + ((first_antenna.x - second_antenna.x) * 2),
                        y: second_antenna.y + ((first_antenna.y - second_antenna.y) * 2),
                    };

                    if is_within_bounds(&antinode_array, first_antinode.x, first_antinode.y) {
                        antinode_array[first_antinode.y as usize][first_antinode.x as usize] = true;
                    }

                    if is_within_bounds(&antinode_array, second_antinode.x, second_antinode.y) {
                        antinode_array[second_antinode.y as usize][second_antinode.x as usize] =
                            true;
                    }
                }
            }
        });

    let answer = antinode_array
        .iter()
        .map(|row| row.iter().filter(|has_antinode| **has_antinode).count())
        .sum();

    Some(answer)
}

fn greatest_common_denominator(a: &isize, b: &isize) -> isize {
    let mut ac = a.clone();
    let mut bc = b.clone();

    while bc != 0 {
        let temp = bc;
        bc = ac % bc;
        ac = temp;
    }
    ac.abs()
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed = parse(input);

    let mut antinode_array: Vec<Vec<bool>> = parsed
        .map
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();

    parsed
        .antennas_by_frequency
        .iter()
        .for_each(|(_, antennas)| {
            for (first_index, first_antenna) in antennas.iter().enumerate() {
                for second_antenna in antennas.iter().skip(first_index + 1) {
                    let diff_x = second_antenna.x - first_antenna.x;
                    let diff_y = second_antenna.y - first_antenna.y;
                    let common_denominator = greatest_common_denominator(&diff_x, &diff_y);

                    let step_x = diff_x / common_denominator;
                    let step_y = diff_y / common_denominator;

                    let mut current_x = first_antenna.x;
                    let mut current_y = first_antenna.y;

                    loop {
                        let possible_origin_of_search_x = current_x - step_x;
                        let possible_origin_of_search_y = current_y - step_y;

                        if is_within_bounds(
                            &antinode_array,
                            possible_origin_of_search_x,
                            possible_origin_of_search_y,
                        ) {
                            current_x = possible_origin_of_search_x;
                            current_y = possible_origin_of_search_y;
                        } else {
                            break;
                        }
                    }

                    while is_within_bounds(&antinode_array, current_x, current_y) {
                        antinode_array[current_y as usize][current_x as usize] = true;
                        current_y += step_y;
                        current_x += step_x;
                    }
                }
            }
        });

    let answer = antinode_array
        .iter()
        .map(|row| row.iter().filter(|has_antinode| **has_antinode).count())
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

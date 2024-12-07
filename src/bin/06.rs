use itertools::Itertools;
use std::cmp::PartialEq;
use std::ops::{Add, AddAssign};

advent_of_code::solution!(6);
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn direction_to_vector(direction: &Direction) -> Vector {
    match direction {
        Direction::Up => Vector { x: 0, y: -1 },
        Direction::Right => Vector { x: 1, y: 0 },
        Direction::Down => Vector { x: 0, y: 1 },
        Direction::Left => Vector { x: -1, y: 0 },
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector {
    x: isize,
    y: isize,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Clone)]
pub struct Guard {
    direction: Direction,
    position: Vector,
}

#[derive(PartialEq)]

enum GuardStatus {
    Step,
    MetObstacle(Vector),
}

pub fn next_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

impl Guard {
    fn move_guard(&mut self, lab: &mut Vec<Vec<char>>) -> Option<GuardStatus> {
        let mut current_status = GuardStatus::Step;

        lab[self.position.y as usize][self.position.x as usize] = 'X';

        let mut next_position = self.position + direction_to_vector(&self.direction);

        if (next_position.y as usize) >= lab.len()
            || next_position.y < 0
            || (next_position.x as usize) >= lab[next_position.y as usize].len()
            || next_position.x < 0
        {
            return None;
        }

        if lab[next_position.y as usize][next_position.x as usize] == '#'
            || lab[next_position.y as usize][next_position.x as usize] == 'O'
        {
            current_status = GuardStatus::MetObstacle(self.position);
            self.direction = next_direction(&self.direction);
            next_position = self.position;
        }

        self.position = next_position;

        Some(current_status)
    }
}

pub struct Input {
    lab: Vec<Vec<char>>,
    guard: Guard,
}

pub fn parse(input: &str) -> Input {
    Input {
        lab: input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' | '#' => c,
                        _ => '.',
                    })
                    .collect_vec()
            })
            .collect_vec(),
        guard: input
            .lines()
            .enumerate()
            .find_map(|(y, l)| {
                l.chars().enumerate().find_map(|(x, c)| match c {
                    '^' => Some(Guard {
                        direction: Direction::Up,
                        position: Vector {
                            x: x as isize,
                            y: y as isize,
                        },
                    }),
                    '>' => Some(Guard {
                        direction: Direction::Right,
                        position: Vector {
                            x: x as isize,
                            y: y as isize,
                        },
                    }),
                    'v' => Some(Guard {
                        direction: Direction::Down,
                        position: Vector {
                            x: x as isize,
                            y: y as isize,
                        },
                    }),
                    '<' => Some(Guard {
                        direction: Direction::Left,
                        position: Vector {
                            x: x as isize,
                            y: y as isize,
                        },
                    }),
                    _ => None,
                })
            })
            .unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parsed = parse(input);

    let mut status_iter = std::iter::from_fn(|| parsed.guard.move_guard(&mut parsed.lab));

    while let Some(_) = status_iter.next() {}

    let answer = parsed
        .lab
        .iter()
        .map(|l| l.iter().map(|c| if *c == 'X' { 1 } else { 0 }).sum::<u32>())
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parsed = parse(input);

    let mut answer = 0;
    let mut guard = parsed.guard.clone();

    while let Some(_) = guard.move_guard(&mut parsed.lab) {
        let obstacle_position = guard.position + direction_to_vector(&guard.direction);

        if (obstacle_position.y as usize) >= parsed.lab.len()
            || obstacle_position.y < 0
            || (obstacle_position.x as usize) >= parsed.lab[obstacle_position.y as usize].len()
            || obstacle_position.x < 0
        {
            break;
        }

        if parsed.lab[obstacle_position.y as usize][obstacle_position.x as usize] == 'X'
            || parsed.lab[obstacle_position.y as usize][obstacle_position.x as usize] == '#'
        {
            continue;
        }

        let mut lab_clone_with_obstacle = parsed.lab.clone();

        lab_clone_with_obstacle[obstacle_position.y as usize][obstacle_position.x as usize] = 'O';

        let mut slow_guard = guard.clone();
        let mut fast_guard = guard.clone();
        while let Some(_) = slow_guard.move_guard(&mut lab_clone_with_obstacle) {
            if let None = fast_guard.move_guard(&mut lab_clone_with_obstacle) {
                break;
            }
            if let None = fast_guard.move_guard(&mut lab_clone_with_obstacle) {
                break;
            }

            if slow_guard.position == fast_guard.position
                && slow_guard.direction == fast_guard.direction
            {
                answer += 1;
                break;
            };
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

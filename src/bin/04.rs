use itertools::Itertools;

advent_of_code::solution!(4);

type Input = Vec<Vec<char>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

pub fn rotate_matrix_90_degrees(matrix: &Input) -> Input {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut rotated_matrix = vec![vec![' '; rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            rotated_matrix[c][rows - r - 1] = matrix[r][c];
        }
    }

    rotated_matrix
}

pub fn check_xmas_from_position(matrix: &Input, x: usize, y: usize) -> u32 {
    let mut matches = 0;

    // Right to left
    if x < matrix[0].len() - 3 {
        if matrix[y][x] == 'X'
            && matrix[y][x + 1] == 'M'
            && matrix[y][x + 2] == 'A'
            && matrix[y][x + 3] == 'S'
        {
            matches += 1;
        }
    }

    // Top left to bottom right
    if x < matrix[y].len() - 3 && y < matrix.len() - 3 {
        if matrix[y][x] == 'X'
            && matrix[y + 1][x + 1] == 'M'
            && matrix[y + 2][x + 2] == 'A'
            && matrix[y + 3][x + 3] == 'S'
        {
            matches += 1;
        }
    }

    matches
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix = parse(input);
    let mut matches = 0;

    for _ in 0..4 {
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                matches += check_xmas_from_position(&matrix, x, y);
            }
        }
        matrix = rotate_matrix_90_degrees(&matrix);
    }

    Some(matches)
}

pub fn check_mas_cross_from(matrix: &Input, x: usize, y: usize) -> u32 {
    if x > 0 && x < matrix[y].len() - 1 && y > 0 && y < matrix.len() - 1 {
        if matrix[y - 1][x - 1] == 'M' && matrix[y][x] == 'A' && matrix[y + 1][x + 1] == 'S' {
            if matrix[y + 1][x - 1] == 'M' && matrix[y][x] == 'A' && matrix[y - 1][x + 1] == 'S' {
                return 1;
            }
        }
    }

    0
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix = parse(input);
    let mut matches = 0;

    for _ in 0..4 {
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                matches += check_mas_cross_from(&matrix, x, y);
            }
        }
        matrix = rotate_matrix_90_degrees(&matrix);
    }

    Some(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

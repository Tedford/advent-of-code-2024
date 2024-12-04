#[allow(dead_code)]
fn pretty_print(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        println!("{:?}", row.iter().collect::<String>());
    }
}
fn count_slice(slice: &[char]) -> i32 {
    let mut count = 0;
    for word in slice.windows(4) {
        count += match word {
            ['X', 'M', 'A', 'S'] => 1,
            _ => 0,
        }
    }
    count
}

fn count_horizontal(matrix: &Vec<Vec<char>>) -> i32 {
    matrix.iter().map(|row| count_slice(row)).sum()
}

fn count_horizontal_inverted(matrix: &Vec<Vec<char>>) -> i32 {
    matrix
        .iter()
        .map(|row| count_slice(&row.iter().rev().cloned().collect::<Vec<char>>()))
        .sum()
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed = vec![vec![' '; matrix.len()]; matrix[0].len()];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }
    transposed
}

fn count_vertical(matrix: &Vec<Vec<char>>) -> i32 {
    count_horizontal(&transpose(matrix))
}

fn count_vertical_inverted(matrix: &Vec<Vec<char>>) -> i32 {
    let mut transposed = transpose(matrix);
    transposed.iter_mut().for_each(|row| row.reverse());
    count_horizontal(&transposed)
}

fn count_diagonal(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let rows = matrix.len();
    let cols = matrix[0].len();

    for row in 0..rows - 3 {
        for col in 0..cols - 3 {
            let word: Vec<char> = (0..4).map(|i| matrix[row + i][col + i]).collect();
            count += match word.as_slice() {
                ['X', 'M', 'A', 'S'] => 1,
                _ => 0,
            }
        }
    }

    count
}

fn count_reverse_diagonal(matrix: &Vec<Vec<char>>) -> i32 {
    let reversed = matrix
        .iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect();
    count_diagonal(&reversed)
}

fn count_diagonal_inverted(matrix: &Vec<Vec<char>>) -> i32 {
    let flipped: Vec<Vec<char>> = matrix.iter().rev().cloned().collect();
    count_diagonal(&flipped)
}

fn count_reverse_diagonal_inverted(matrix: &Vec<Vec<char>>) -> i32 {
    let mut flipped: Vec<Vec<char>> = matrix.iter().rev().cloned().collect();
    flipped.iter_mut().for_each(|row| row.reverse());
    count_diagonal(&flipped)
}

pub fn part1(input: &Vec<String>) -> i32 {
    let matrix = input.iter().map(|line| line.chars().collect()).collect();

    count_horizontal(&matrix)
        + count_horizontal_inverted(&matrix)
        + count_vertical(&matrix)
        + count_vertical_inverted(&matrix)
        + count_diagonal(&matrix)
        + count_reverse_diagonal(&matrix)
        + count_diagonal_inverted(&matrix)
        + count_reverse_diagonal_inverted(&matrix)
}

pub fn part2(input: &Vec<String>) -> i32 {
    let matrix: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let rows = matrix.len();
    let cols = matrix[0].len();

    for row in 0..rows - 2 {
        for col in 0..cols - 2 {
            if matrix[row + 1][col + 1] == 'A'
                && (
                    // 1
                    ((matrix[row][col] == 'M' && matrix[row][col + 2] == 'M') && (matrix[row + 2][col] == 'S' && matrix[row + 2][col + 2] == 'S')) ||
                    // 2
                    ((matrix[row][col] == 'S' && matrix[row][col + 2] == 'M') && (matrix[row + 2][col] == 'S' && matrix[row + 2][col + 2] == 'M')) ||
                    // 3
                    ((matrix[row][col] == 'S' && matrix[row][col + 2] == 'S') && (matrix[row + 2][col] == 'M' && matrix[row + 2][col + 2] == 'M')) ||
                    // 4
                    ((matrix[row][col] == 'M' && matrix[row][col + 2] == 'S') && (matrix[row + 2][col] == 'M' && matrix[row + 2][col + 2] == 'S'))
                )
            {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
            .lines()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn horizontal_valid() {
        let sample = get_sample();
        let matrix = sample.iter().map(|line| line.chars().collect()).collect();
        let result = count_horizontal(&matrix);
        assert_eq!(result, 3);
    }

    #[test]
    fn horizontal_inverse_valid() {
        let sample = get_sample();
        let matrix = sample.iter().map(|line| line.chars().collect()).collect();
        let result = count_horizontal_inverted(&matrix);
        assert_eq!(result, 2);
    }

    #[test]
    fn vertical_valid() {
        let sample = get_sample();
        let matrix = sample.iter().map(|line| line.chars().collect()).collect();
        let result = count_vertical(&matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn vertical_inverted_valid() {
        let sample = get_sample();
        let matrix = sample.iter().map(|line| line.chars().collect()).collect();
        let result = count_vertical_inverted(&matrix);
        assert_eq!(result, 2);
    }

    #[test]
    fn diagonal_valid() {
        let sample = get_sample();
        let matrix = sample.iter().map(|line| line.chars().collect()).collect();
        let result = count_diagonal(&matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn reverse_diagonal_valid() {
        let sample = get_sample();
        let matrix = sample.iter().map(|line| line.chars().collect()).collect();
        let result = count_reverse_diagonal(&matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn diagonal_inverted_valid() {
        let sample = get_sample();
        let matrix = sample.iter().map(|line| line.chars().collect()).collect();
        let result = count_diagonal_inverted(&matrix);
        assert_eq!(result, 4);
    }

    #[test]
    fn reverse_diagonal_inverted_valid() {
        let sample = get_sample();
        let matrix = sample.iter().map(|line| line.chars().collect()).collect();
        let result = count_reverse_diagonal_inverted(&matrix);
        assert_eq!(result, 4);
    }

    #[test]
    fn sample_part1_valid() {
        let sample = get_sample();
        let result = part1(&sample);
        assert_eq!(result, 18);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 9);
    }
}

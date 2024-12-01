use std::collections::HashMap;

fn calculate_distances(x: &Vec<i32>, y: &Vec<i32>) -> Vec<i32> {
    x.iter().zip(y.iter()).map(|(a, b)| (a - b).abs()).collect()
}

fn parse_input(input: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            left.push(parts[0].parse::<i32>().unwrap());
            right.push(parts[1].parse::<i32>().unwrap());
        }
    }
    (left, right)
}

fn get_frequency(number: i32, input: &Vec<i32>) -> i32 {
    input.iter().filter(|x|x == &&number).count() as i32
}

pub fn part1(input: &Vec<String>) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    calculate_distances(&left, &right).iter().sum()
}
pub fn part2(input: &Vec<String>) -> i32 {
    let (left, right) = parse_input(input);

    let mut frequencies = HashMap::new();

    left.iter().for_each(|x| {
        if !( frequencies.contains_key(x)) {
            frequencies.insert(x, get_frequency(*x, &right));
        }
    });

    left.iter().map(|x| x * frequencies.get(x).unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> String {
        r#"3   4
4   3
2   5
1   3
3   9
3   3"#.to_string()
    }

    #[test]
    fn sample1_passes() {
        let sample = get_sample();
        let result = part1(&sample.lines().map(|x| x.to_string()).collect());
        assert_eq!(result, 11);
    }

    #[test]
    fn sample2_passes() {
        let sample = get_sample();
        let result = part2(&sample.lines().map(|x| x.to_string()).collect());
        assert_eq!(result, 31);
    }
}

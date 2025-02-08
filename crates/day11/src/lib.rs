use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
struct Stone {
    value: i64,
    count: i64,
}

fn parse_stones(input: &Vec<String>) -> Vec<Stone> {
    input.iter().map(|x| x.split_whitespace()).flatten().map(|x| Stone { value: x.parse::<i64>().unwrap(), count: 1 }).collect()
}

pub fn blink(stones: &Vec<i64>) -> Vec<i64> {
    let mut result = vec![];

    for stone in stones.clone() {
        if stone == 0 {
            result.push(1);
        } else {
            let text = stone.to_string();
            if text.len() % 2 == 0 {
                let left = text[0..text.len() / 2].parse::<i64>().unwrap();
                let right = text[text.len() / 2..text.len()].parse::<i64>().unwrap();

                result.push(left);
                result.push(right);
            } else {
                result.push(stone * 2024);
            }
        }
    }

    result
}

pub fn part1(input: &Vec<String>) -> i64 {
    let mut stones = parse_stones(&input);
    for _ in 0..25 {
        stones = blink_and_condense(&stones);
    }

    stones.iter().map(|s| s.count).sum()
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut stones = parse_stones(&input);
    for _ in 0..75 {
        stones = blink_and_condense(&stones);
    }

    stones.iter().map(|s| s.count).sum()
}

fn blink_and_condense(stones: &Vec<Stone>) -> Vec<Stone> {
    let mut cache: HashMap<i64, i64> = HashMap::new();

    for stone in stones {
        match stone.value {
            0 => *cache.entry(1).or_insert(0) += stone.count,
            x if x.to_string().len() % 2 == 0 => {
                let text = x.to_string();
                let left = text[0..text.len() / 2].parse::<i64>().unwrap();
                let right = text[text.len() / 2..text.len()].parse::<i64>().unwrap();
                *cache.entry(left).or_insert(0) += stone.count;
                *cache.entry(right).or_insert(0) += stone.count;
            },
            x => {
                let key = x * 2024;
                *cache.entry(key).or_insert(0) += stone.count;
            }
        }
    }
    cache.into_iter().map(|(value, count)| Stone { value, count }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        include_str!("sample.dat")
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    #[test]
    fn single_blink() {
        let stones = vec![0, 1, 10, 99, 999];
        let result = blink(&stones);
        assert_eq!(result, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn sample_part1_full() {
        let sample = get_sample();
        let result = part1(&sample);
        assert_eq!(result, 55312);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 65601038650482);
    }
}

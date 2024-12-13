fn parse(input: &Vec<String>) -> Vec<i64> {
    input.iter().map(|x| x.split_whitespace()).flatten().map(|x| x.parse::<i64>().unwrap()).collect()
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

fn blinks(stones: Vec<i64>, cycles: i32) -> Vec<i64> {
    let mut result = stones;

    print!("Starting:");
    for i in 0..cycles {
        if i % 5 == 0 {
            print!("..{cycles}")
        }
        result = blink(&result);
    }
    println!("Done");
    result
}

pub fn part1(input: &Vec<String>) -> i64 {
    let stones = blinks(parse(input), 25);
    stones.len() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut total = 0;
    for stone in parse(input) {
        total += blinks(vec![stone], 75).len() as i64;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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

    #[rstest]
    #[case::one_blink(1, vec![125, 17], vec![253000, 1, 7])]
    #[case::two_blinks(2, vec![125, 17], vec![253, 0, 2024, 14168])]
    #[case::three_blinks(3, vec![125, 17], vec![512072, 1, 20, 24, 28676032])]
    #[case::four_blinks(4, vec![125, 17], vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032])]
    #[case::five_blinks(5, vec![125, 17], vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
    )]
    #[case::six_blinks(6, vec![125, 17], vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]
    )]
    fn blinks_valid(#[case]cycles: i32, #[case] start: Vec<i64>, #[case] target: Vec<i64>) {
        assert_eq!(blinks(start, cycles), target);
    }

    #[test]
    fn blink_length() {
        let result = blinks(vec![125i64, 17i64], 6);
        println!("{:?}", result);
        assert_eq!(result.len(), 22);
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
        assert_eq!(result, 0);
    }
}

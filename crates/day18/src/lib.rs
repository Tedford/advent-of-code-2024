#[derive(Clone)]
enum Address {
    Clear,
    Corrupted,
    Walked,
}

struct Coordinate {
    x: usize,
    y: usize,
}


fn parse(input: &Vec<String>) -> Vec<Coordinate> {
    input.iter().map(|l| {
        match l.split(",").collect::<Vec<&str>>().as_slice() {
            [left, right] => Coordinate { x: left.parse::<usize>().unwrap(), y: right.parse::<usize>().unwrap() },
            _ => panic!("Unexpected format {} detected", l)
        }
    }).collect()
}

fn create_map(x: usize, y: usize, drops: &Vec<Coordinate>, preview: usize) -> Vec<Vec<Address>> {
    let mut space = vec![vec![Address::Clear; x]; y];

    drops.iter().take(preview).for_each(|d|
        space[d.y][d.x] = Address::Corrupted
    );

    space
}

fn find_exit(drops: &Vec<Coordinate>, bound: (usize, usize), preview: usize) -> i64 {
    let map = create_map(bound.0, bound.1, drops, preview);
    navigate(&map, bound)
}

fn navigate(space: &Vec<Vec<Address>>, exit: (usize, usize)) -> i64 {
    let mut steps = 0;


    steps
}


pub fn part1(input: &Vec<String>) -> i64 {
    let drops = parse(input);
    input.len() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {
    input.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        include_str!("sample.dat")
            .lines()
            .map(|l| l.to_string())
            .collect()
    }

    #[test]
    fn sample_valid() {
        let sample = parse(&get_sample());
        let result = find_exit(&sample, (12, 12), 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_sample()), 1);
    }
}
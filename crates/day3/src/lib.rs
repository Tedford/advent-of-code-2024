use regex::Regex;
#[derive(Debug)]
struct Operation {
    op: String,
    x: i32,
    y: i32,
}

impl Operation {
    fn execute(&self) -> i32 {
        match self.op.as_str() {
            "mul" => self.x * self.y,
            _ => 0,
        }
    }
}

fn parse_operands(input: &String) -> Vec<Operation> {
    let re = Regex::new(r"(?P<op>mul)\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let op = cap.name("op").unwrap().as_str().to_string();
            let x = cap.name("x").unwrap().as_str().parse::<i32>().unwrap();
            let y = cap.name("y").unwrap().as_str().parse::<i32>().unwrap();
            Operation { op, x, y }
        })
        .collect()
}

fn conditionally_parse_operands(input: &String) -> Vec<Operation> {
    let mut conditional = String::new();
    let mut remainder = input.clone();

    while !remainder.is_empty() {
        match remainder.find("don't()") {
            Some(i) => {
                conditional.push_str(&remainder[..i]);
                remainder = remainder[i + 7..].to_string();

                match remainder.find("do()") {
                    Some(j) => {
                        remainder = remainder[j + 4..].to_string();
                    }
                    None => {
                        remainder.truncate(0);
                    }
                }
            }
            None => {
                conditional.push_str(&remainder);
                remainder.truncate(0);
            }
        }
    }

    parse_operands(&conditional)
}

pub fn part1(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|x| parse_operands(x))
        .flatten()
        .map(|o| o.execute())
        .sum()
}

pub fn part2(input: &Vec<String>) -> i32 {
    conditionally_parse_operands(&input.join(""))
        .iter()
        .map(|o| o.execute())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#
            .lines()
            .map(|x| x.to_string())
            .collect()
    }

    fn get_sample2() -> Vec<String> {
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
            .lines()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn sample_first_passes() {
        let sample = get_sample();
        let result = part1(&sample);
        assert_eq!(result, 161);
    }

    #[test]
    fn sample_second_passes() {
        let sample = get_sample2();
        let result = part2(&sample);
        assert_eq!(result, 48);
    }
}

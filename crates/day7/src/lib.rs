struct Calibration {
    target: i64,
    values: Vec<i64>,
}

fn get_calibration_rules(input: &String) -> Calibration {
    let [left, right] = input.split(":").map(|x| x.trim()).collect::<Vec<_>>()[..] else {
        todo!()
    };
    let target = left.parse::<i64>().unwrap();
    let values = right
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    Calibration { target, values }
}

fn is_valid(calibration: &Calibration) -> bool {
    let mut valid = false;
    let mut equations = Vec::new();

    let cases = 2_i32.pow((calibration.values.len() - 1) as u32);
    for i in 0..cases {
        let mut total = calibration.values[0];
        let mut formula = format!("{}", total);

        for j in 1..calibration.values.len() {
            if (i >> (j - 1)) & 1 == 1 {
                total += calibration.values[j];
                formula.push_str(&format!(" + {}", calibration.values[j]));
            } else {
                total *= calibration.values[j];
                formula.push_str(&format!(" * {}", calibration.values[j]));
            }
        }

        valid |= total == calibration.target;
        if total == calibration.target {
            equations.push(format!("\t{} == {}", total, formula));
        }
    }

    if valid {
        println!("[{:?}] == {:?}", calibration.target, calibration.values);
    }

    valid
}


pub fn part1(_input: &Vec<String>) -> i64 {
    let rules = _input
        .iter()
        .map(|x| get_calibration_rules(x))
        .collect::<Vec<_>>();

    rules.iter().filter(|i| is_valid(i)).map(|i| i.target).sum()
}

pub fn part2(_input: &Vec<String>) -> i64 {
    0
}

mod tests {
    use super::*;
    use rstest::rstest;

    fn get_sample() -> Vec<String> {
        include_str!("sample.dat")
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    #[rstest]
    #[case(971, vec![64, 555, 4, 23, 14, 225, 86], true)]
    #[case(122, vec![9, 32, 72, 9, 1], true)]
    #[case(503, vec![358, 1, 9, 78, 58], true)]
    fn formula_validation(#[case] target: i64, #[case] values: Vec<i64>, #[case] expected: bool) {
        let calibration = Calibration { target, values };
        let result = is_valid(&calibration);
        assert_eq!(result, expected);
    }

    #[test]
    fn sample_part1() {
        let input = get_sample();
        let result = part1(&input);
        assert_eq!(result, 3749);
    }

    #[test]
    fn sample_part2() {
        let input = get_sample();
        let result = part2(&input);
        assert_eq!(result, 11387);
    }
}

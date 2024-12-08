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

fn is_valid(calibration: &Calibration, operands: Vec<char>) -> bool {
    let cases = operands.len().pow((calibration.values.len() - 1) as u32);

    (0..cases).any(|i| {
        let mut total = calibration.values[0];
        let mut formula = format!("{}", total);

        for (j, &value) in calibration.values.iter().enumerate().skip(1) {
            let operand = operands[(i / operands.len().pow((j - 1) as u32)) % operands.len()];
            total = match operand {
                '+' => total + value,
                '|' => format!("{total}{}", value).parse::<i64>().unwrap(),
                '*' => total * value,
                _ => panic!("Invalid operand"),
            };
            formula.push_str(&format!(" {operand} {}", value));
        }

        if total == calibration.target {
            println!("[{:?}] == {:?}", calibration.target, calibration.values);
            println!("\t{} == {}", total, formula);
            true
        } else {
            false
        }
    })
}

fn kernel(input: &Vec<String>, operands: &Vec<char>) -> i64 {
    let rules = input
        .iter()
        .map(|x| get_calibration_rules(x))
        .collect::<Vec<_>>();

    rules
        .iter()
        .filter(|i| is_valid(i, operands.clone()))
        .map(|i| i.target)
        .sum()
}

pub fn part1(input: &Vec<String>) -> i64 {
    kernel(input, &vec!['+', '*'])
}

pub fn part2(input: &Vec<String>) -> i64 {
    kernel(input, &vec!['+', '*', '|'])
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
    #[case::target_971(971, vec![64, 555, 4, 23, 14, 225, 86], true)]
    #[case::target_122(122, vec![9, 32, 72, 9, 1], true)]
    #[case::target_503(503, vec![358, 1, 9, 78, 58], true)]
    fn formula_validation_part1(
        #[case] target: i64,
        #[case] values: Vec<i64>,
        #[case] expected: bool,
    ) {
        let calibration = Calibration { target, values };
        let result = is_valid(&calibration, vec!['+', '*']);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::target_971(971, vec![64, 555, 4, 23, 14, 225, 86], true)]
    #[case::target_122(122, vec![9, 32, 72, 9, 1], true)]
    #[case::target_503(503, vec![358, 1, 9, 78, 58], true)]
    #[case::target_156(156, vec![15, 6], true)]
    #[case::target_7290(7290, vec![6,8,6,15], true)]
    #[case::target_192(192, vec![17,8,14], true)]
    fn formula_validation_part2(
        #[case] target: i64,
        #[case] values: Vec<i64>,
        #[case] expected: bool,
    ) {
        let calibration = Calibration { target, values };
        let result = is_valid(&calibration, vec!['+', '*', '|']);
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

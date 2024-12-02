#[derive(Eq, PartialEq, Hash)]
enum Change {
    None,
    Increasing,
    Decreasing,
}

fn parse_report(report: &String) -> Vec<i32> {
    report
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn analyze(report: &Vec<i32>) -> Vec<(Change, i32)> {
    report
        .windows(2)
        .map(|window| {
            (
                match window[0] - window[1] {
                    0 => Change::None,
                    x if x > 0 => Change::Decreasing,
                    x if x < 0 => Change::Increasing,
                    _ => Change::None,
                },
                (window[1] - window[0]).abs(),
            )
        })
        .collect()
}

fn is_safe(reports: &Vec<i32>) -> bool {
    let windows = analyze(reports);

    let unidirectional = windows
        .iter()
        .map(|(first, _)| first)
        .collect::<std::collections::HashSet<_>>()
        .len()
        == 1;

    let within_tolerance = windows.iter().map(|(_, second)| second).all(|x| *x < 4);

    unidirectional && within_tolerance
}

fn is_safe_with_damper(reports: &Vec<i32>) -> bool {
    match is_safe(reports) {
        true => true,
        false => reports.iter().enumerate().any(|(i, _)| {
            let mut slice = reports.clone();
            slice.remove(i);
            is_safe(&slice)
        }),
    }
}

pub fn part1(input: &Vec<String>) -> i32 {
    let reports = input.iter().map(|x| parse_report(x)).collect::<Vec<_>>();
    reports.iter().filter(|x| is_safe(x)).count() as i32
}

pub fn part2(_input: &Vec<String>) -> i32 {
    let reports = _input.iter().map(|x| parse_report(x)).collect::<Vec<_>>();
    reports.iter().filter(|x| is_safe_with_damper(x)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#
            .lines()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn sample_first_passes() {
        let sample = get_sample();
        let result = part1(&sample);
        assert_eq!(result, 2);
    }

    #[test]
    fn sample_second_passes() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 4);
    }
}

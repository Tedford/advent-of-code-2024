#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Play {
    a: i32,
    b: i32,
}

impl Play {
    fn calculate_tokens(&self) -> i32 {
        self.a * 3 + self.b
    }
}


fn parse_button(line: &&str) -> (usize, usize) {
    let parts: Vec<_> = line.trim().split(", ").map(|s| s.to_string()).collect();
    let x = match parts[0].split("+").collect::<Vec<&str>>().as_slice() {
        ["X", x] => x.parse::<usize>().unwrap(),
        x => panic!("Unexpected value {:?}", x)
    };
    let y = match parts[1].split("+").collect::<Vec<&str>>().as_slice() {
        ["Y", x] => x.parse::<usize>().unwrap(),
        x => panic!("Unexpected value {:?}", x)
    };

    (x, y)
}

fn parse_prize(line: &&str, scalar: usize) -> (usize, usize) {
    let parts: Vec<_> = line.trim().split(", ").map(|s| s.to_string()).collect();
    let x = match parts[0].split("=").collect::<Vec<&str>>().as_slice() {
        ["X", x] => x.parse::<usize>().unwrap() + scalar,
        x => panic!("Unexpected value {:?}", x)
    };
    let y = match parts[1].split("=").collect::<Vec<&str>>().as_slice() {
        ["Y", x] => x.parse::<usize>().unwrap() + scalar,
        x => panic!("Unexpected value {:?}", x)
    };

    (x, y)
}

fn parse(input: &Vec<String>, scalar: usize) -> Vec<Machine> {
    let mut result = vec![];

    let mut a = (0, 0);
    let mut b = (0, 0);

    for l in input.iter() {
        let parts: Vec<&str> = l.split(":").collect();
        match parts.as_slice() {
            ["Button A", x] => {
                a = parse_button(x);
            }
            ["Button B", x] => {
                b = parse_button(x);
            }
            ["Prize", x] => {
                result.push(Machine { a, b, prize: parse_prize(x, scalar) });
            }
            _ => {}
        }
    }


    result
}

fn is_win(target: (usize, usize), a: (usize, usize), b: (usize, usize), limit: usize) -> Option<Play> {
    let mut options = vec![];

    println!("Target: ({},{})", target.0, target.1);
    for i in 0..std::cmp::min(std::cmp::min(target.0 / a.0, target.1 / a.1), limit) {
        let new_x = target.0 - a.0 * i;
        let new_y = target.1 - a.1 * i;

        let x_presses = new_x as f32 / b.0 as f32;
        let y_presses = new_y as f32 / b.1 as f32;
        if x_presses == y_presses && x_presses.fract() == 0.0 && (x_presses.trunc() as i32) < 101 {
            let play = Play { a: i as i32, b: new_x as i32 / b.0 as i32 };
            println!("\tA: [{}] => ({},{}) B: [{}] => ({},{}) ... [{}]", play.a, a.0, a.1, play.b, b.0, b.1, play.calculate_tokens());
            options.push(play)
        }
    }

    if options.len() > 0 {
        options.sort_by(|a, b| a.calculate_tokens().cmp(&b.calculate_tokens()));
        let winner = options.first().unwrap().clone();
        println!("\tWinner {:?}->{}", winner, winner.calculate_tokens());
        Some(winner)
    } else {
        None
    }
}

fn get_cheapest_win(machine: &Machine, limit: usize) -> Option<i32> {
    match is_win(machine.prize, machine.a, machine.b, limit) {
        Some(p) => Some(p.calculate_tokens()),
        _ => None
    }
}

pub fn part1(input: &Vec<String>) -> i64 {
    let machines = parse(&input, 0);
    machines.iter().map(|m| get_cheapest_win(m, 101)).map(|w| match w {
        Some(x) => x as i64,
        _ => 0
    }).sum()
}

pub fn part2(input: &Vec<String>) -> i64 {
    // let machines = parse(&input, 10000000000000);
    // machines.iter().map(|m| get_cheapest_win(m, usize::MAX)).map(|w| match w {
    //     Some(x) => x as i64,
    //     _ => 0
    // }).sum()
    input.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((2048, 864), (42, 61), (94, 31))]
    fn validate_machine(#[case]target: (usize, usize), #[case] a: (usize, usize), #[case] b: (usize, usize)) {
        let result = is_win(target, a, b, 101);
        assert_eq!(result, None);
    }

    fn get_sample() -> Vec<String> {
        include_str!("sample.dat")
            .lines()
            .map(|l| l.to_string())
            .collect()
    }

    #[test]
    fn sample_part1_valid() {
        let input = get_sample();
        let result = part1(&input);
        assert_eq!(480, result);
    }
    #[test]
    fn sample_part2_valid() {
        let input = get_sample();
        let result = part2(&input);
        assert_eq!(0, result);
    }
}

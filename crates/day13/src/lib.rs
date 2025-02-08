#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Play {
    a: i64,
    b: i64,
}

impl Play {
    fn calculate_tokens(&self) -> i64 {
        self.a * 3 + self.b
    }
}

fn parse_button(line: &&str) -> (usize, usize) {
    let parts: Vec<_> = line.trim().split(", ").map(|s| s.to_string()).collect();
    let x = match parts[0].split("+").collect::<Vec<&str>>().as_slice() {
        ["X", x] => x.parse::<usize>().unwrap(),
        x => panic!("Unexpected value {:?}", x),
    };
    let y = match parts[1].split("+").collect::<Vec<&str>>().as_slice() {
        ["Y", x] => x.parse::<usize>().unwrap(),
        x => panic!("Unexpected value {:?}", x),
    };

    (x, y)
}

fn parse_prize(line: &&str, scalar: usize) -> (usize, usize) {
    let parts: Vec<_> = line.trim().split(", ").map(|s| s.to_string()).collect();
    let x = match parts[0].split("=").collect::<Vec<&str>>().as_slice() {
        ["X", x] => x.parse::<usize>().unwrap() + scalar,
        x => panic!("Unexpected value {:?}", x),
    };
    let y = match parts[1].split("=").collect::<Vec<&str>>().as_slice() {
        ["Y", x] => x.parse::<usize>().unwrap() + scalar,
        x => panic!("Unexpected value {:?}", x),
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

fn has_solution(matrix: [[u64; 2]; 2], vector: [u64; 2]) -> Option<Play> {
    // NM = Coefficient Matrix^-1 * Prize Vector
    let xa = matrix[0][0] as i64;
    let xb = matrix[0][1] as i64;
    let ya = matrix[1][0] as i64;
    let yb = matrix[1][1] as i64;
    let prize_x = vector[0] as i64;
    let prize_y = vector[1] as i64;

    // Calculate the determinant
    let determinant = xa * yb - xb * ya;

    if determinant == 0 {
        // Determinant is zero: no solutions
        return None;
    }

    // Compute the inverse matrix components (scaled by determinant)
    let inv_xa = yb;
    let inv_xb = -xb;
    let inv_ya = -ya;
    let inv_yb = xa;

    // Multiply the inverse matrix by the prize vector
    let n = (inv_xa * prize_x + inv_xb * prize_y) / determinant;
    let m = (inv_ya * prize_x + inv_yb * prize_y) / determinant;

    // Check if n and m are valid (non-negative and integer)
    if n < 0 || m < 0 || (inv_xa * prize_x + inv_xb * prize_y) % determinant != 0 || (inv_ya * prize_x + inv_yb * prize_y) % determinant != 0 {
        return None;
    }

    Some(Play { a: n, b: m })
}

fn calculate_win_costs(m: &Machine) -> i64 {
    match has_solution([[m.a.0 as u64, m.b.0 as u64], [m.a.1 as u64, m.b.1 as u64]], [m.prize.0 as u64, m.prize.1 as u64]) {
        Some(x) => x.calculate_tokens(),
        _ => 0,
    }
}

pub fn part1(input: &Vec<String>) -> i64 {
    parse(&input, 0).iter().map(|m| calculate_win_costs(m)).sum()
}

pub fn part2(input: &Vec<String>) -> i64 {
    parse(&input, 10000000000000).iter().map(|m| calculate_win_costs(m)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn get_sample() -> Vec<String> {
        include_str!("sample.dat").lines().map(|l| l.to_string()).collect()
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
        assert_eq!(875318608908, result);
    }
}

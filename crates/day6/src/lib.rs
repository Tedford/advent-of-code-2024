#[derive(Clone, PartialEq, Eq)]
enum Location {
    Empty,       // .
    Obstacle,    // #
    Patrolled,   // X
    PatrolNorth, // ^
    PatrolSouth, // v
    PatrolEast,  // >
    PatrolWest,  // <
}

impl Location {
    fn as_str(&self) -> &str {
        match self {
            Location::Empty => ".",
            Location::Obstacle => "#",
            Location::Patrolled => "X",
            Location::PatrolNorth => "^",
            Location::PatrolSouth => "v",
            Location::PatrolEast => ">",
            Location::PatrolWest => "<",
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn build_map(input: &Vec<String>) -> (Vec<Vec<Location>>, (usize, usize, Location)) {
    let mut map = Vec::new();
    let mut start = (0, 0, Location::Empty);
    for line in input {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(Location::Empty),
                '#' => row.push(Location::Obstacle),
                '^' => {
                    start = (map.len(), row.len(), Location::PatrolNorth);
                    row.push(Location::PatrolNorth);
                }
                'v' => {
                    start = (map.len(), row.len(), Location::PatrolSouth);
                    row.push(Location::PatrolSouth);
                }
                '>' => {
                    start = (map.len(), row.len(), Location::PatrolEast);
                    row.push(Location::PatrolEast);
                }
                '<' => {
                    start = (map.len(), row.len(), Location::PatrolWest);
                    row.push(Location::PatrolWest);
                }
                _ => panic!("Invalid character in input"),
            }
        }
        map.push(row);
    }
    (map, start)
}

fn walk(map: &Vec<Vec<Location>>, start: &(usize, usize, Location)) -> (Vec<Vec<Location>>, bool) {
    let mut new_map = map.clone();
    let mut done = false;
    let mut circular = false;
    let mut current = (start.0, start.1);
    // let mut path = Vec::new();
    let mut steps = 0;
    let elements = (map.len() * map[0].len()) as i32;

    while !done {
        // path.push(current.clone());
        done = match new_map[current.0][current.1] {
            Location::PatrolNorth => {
                if current.0 > 0 {
                    if new_map[current.0 - 1][current.1] == Location::Obstacle {
                        new_map[current.0][current.1] = Location::PatrolEast;
                    } else {
                        new_map[current.0][current.1] = Location::Patrolled;
                        current = (current.0 - 1, current.1);
                        new_map[current.0][current.1] = Location::PatrolNorth;
                    }
                    false
                } else {
                    true
                }
            }
            Location::PatrolSouth => {
                if current.0 < map.len() - 1 {
                    if new_map[current.0 + 1][current.1] == Location::Obstacle {
                        new_map[current.0][current.1] = Location::PatrolWest;
                    } else {
                        new_map[current.0][current.1] = Location::Patrolled;
                        current = (current.0 + 1, current.1);
                        new_map[current.0][current.1] = Location::PatrolSouth;
                    }
                    false
                } else {
                    true
                }
            }
            Location::PatrolEast => {
                if current.1 < map[current.0].len() - 1 {
                    if new_map[current.0][current.1 + 1] == Location::Obstacle {
                        new_map[current.0][current.1] = Location::PatrolSouth;
                    } else {
                        new_map[current.0][current.1] = Location::Patrolled;
                        current = (current.0, current.1 + 1);
                        new_map[current.0][current.1] = Location::PatrolEast;
                    }
                    false
                } else {
                    true
                }
            }
            Location::PatrolWest => {
                if current.1 > 0 {
                    if new_map[current.0][current.1 - 1] == Location::Obstacle {
                        new_map[current.0][current.1] = Location::PatrolNorth;
                    } else {
                        new_map[current.0][current.1] = Location::Patrolled;
                        current = (current.0, current.1 - 1);
                        new_map[current.0][current.1] = Location::PatrolWest;
                    }
                    false
                } else {
                    true
                }
            }
            _ => panic!(
                "The map is in an invalid state.  Unexpected character {}",
                new_map[current.0][current.1]
            ),
        };

        steps += 1;

        match steps > elements {
            true => {
                done = true;
                circular = true;
            }
            false => {}
        }

        // println!();
        // aoc::pretty_print(&new_map);
    }
    new_map[current.0][current.1] = Location::Patrolled;

    (new_map.clone(), circular)
}

fn count_state(map: &Vec<Vec<Location>>, state: Location) -> i32 {
    map.iter()
        .map(|row| row.iter().filter(|x| x == &&state))
        .flatten()
        .count() as i32
}

pub fn part1(input: &Vec<String>) -> i32 {
    let (map, start) = build_map(input);
    let (patrolled, _) = walk(&map, &start);

    count_state(&patrolled, Location::Patrolled)
}

pub fn part2(input: &Vec<String>) -> i32 {
    let (map, start) = build_map(input);
    let (mut patrolled, _) = walk(&map, &start);

    let mut options = 0;

    let reset = start.clone();
    patrolled[reset.0][reset.1] = reset.2;
    for row in 0..patrolled.len() {
        for col in 0..patrolled[row].len() {
            if patrolled[row][col] == Location::Patrolled {
                let mut map2 = map.clone();
                map2[row][col] = Location::Obstacle;
                options += match walk(&map2, &start) {
                    (_, true) => 1,
                    _ => 0,
                }
            }
        }
    }

    options
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
    fn sample_part1_valid() {
        let sample = get_sample();
        let result = part1(&sample);
        assert_eq!(result, 41);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 6);
    }
}

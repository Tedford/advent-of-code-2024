use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
struct Trailhead {
    x: usize,
    y: usize,
    count: i32,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Trail {
    start: (usize, usize),
    end: (usize, usize),
    path: Vec<(usize, usize)>,
}

#[allow(dead_code)]
fn pretty_print_path(trail: &Trail) {
    let path = trail.path.clone();
    let rows = path.iter().map(|(x, _)| x).max().unwrap();
    let cols = path.iter().map(|(_, y)| y).max().unwrap();

    let mut matrix = vec![vec!['.'; rows + 1]; cols + 1];
    path.iter().enumerate().for_each(|(i, (x, y))| {
        matrix[*y][*x] = std::char::from_digit(i as u32, 10).unwrap();
    });
    aoc::pretty_print(&matrix);
}

fn to_matrix(input: &Vec<String>) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

fn walk_trail(map: &Vec<Vec<i32>>, path: Vec<(usize, usize)>, x: usize, y: usize) -> Vec<Trail> {
    let target = match map[y][x] {
        n if n < 9 => n + 1,
        _ => -1,
    };

    if target != -1 {
        let mut result = HashSet::new();

        if x > 0 {
            explore_direction(map, &path, x, y, target, &mut result, -1, 0);
        }
        if x + 1 < map[0].len() {
            explore_direction(map, &path, x, y, target, &mut result, 1, 0);
        }
        if y > 0 {
            explore_direction(map, &path, x, y, target, &mut result, 0, -1);
        }
        if y + 1 < map.len() {
            explore_direction(map, &path, x, y, target, &mut result, 0, 1);
        }

        result.iter().map(|p| p.clone()).collect()
    } else {
        vec![Trail {
            start: path.first().unwrap().clone(),
            end: path.last().unwrap().clone(),
            path,
        }]
    }
}

fn explore_direction(
    map: &Vec<Vec<i32>>,
    path: &Vec<(usize, usize)>,
    x: usize,
    y: usize,
    target: i32,
    result: &mut HashSet<Trail>,
    dx: isize,
    dy: isize,
) {
    let new_x = (x as isize + dx) as usize;
    let new_y = (y as isize + dy) as usize;
    if new_x < map[0].len() && new_y < map.len() && map[new_y][new_x] == target {
        let mut next = path.clone();
        next.push((new_x, new_y));
        walk_trail(map, next, new_x, new_y).iter().for_each(|p| {
            result.insert(p.clone());
        });
    }
}

fn mark_singular_trails(input: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<Trail> {
    let trails = mark_distinct_trails(input, x, y);
    let mut hash = HashSet::new();
    let mut unique: Vec<Trail> = Vec::new();

    trails.iter().for_each(|trail| {
        let key = (trail.start, trail.end);
        match hash.insert(key) {
            true => unique.push(trail.clone().clone()),
            _ => {}
        };
    });

    unique
}

fn mark_distinct_trails(input: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<Trail> {
    walk_trail(input, vec![(x, y)], x, y)
        .into_iter()
        .filter(|trail| trail.path.len() == 10)
        .collect::<Vec<_>>()
}

pub fn part1(input: &Vec<String>) -> i64 {
    calculate_trailheads(input, mark_singular_trails)
}

pub fn part2(input: &Vec<String>) -> i64 {
    calculate_trailheads(input, mark_distinct_trails)
}

fn calculate_trailheads<F>(input: &Vec<String>, trail_fn: F) -> i64
where
    F: Fn(&Vec<Vec<i32>>, usize, usize) -> Vec<Trail>,
{
    let topographical_map = to_matrix(input);
    let mut trailheads = Vec::new();
    for (y, line) in topographical_map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &0 {
                let paths = trail_fn(&topographical_map, x, y);
                trailheads.push(Trailhead {
                    x,
                    y,
                    count: paths.len() as i32,
                });
            }
        }
    }
    trailheads.iter().map(|x| x.count).sum::<i32>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        include_str!("sample.dat").lines().map(|line| line.to_string()).collect()
    }

    #[test]
    fn sample_part1_valid() {
        let sample = get_sample();
        let result = part1(&sample);
        assert_eq!(result, 36);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 81);
    }
}

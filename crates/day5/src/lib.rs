use std::collections::HashMap;

fn get_rules(input: &[String]) -> HashMap<i32, Vec<i32>> {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in input {
        let [page, dependent]: [i32; 2] = line
            .split("|")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        if rules.contains_key(&page) {
            let rule = rules.get_mut(&page).unwrap();
            rule.push(dependent);
        } else {
            rules.insert(page, vec![dependent]);
        }
    }
    rules
}

fn parse(input: &Vec<String>) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let divider = input.iter().position(|line| line == "").unwrap();
    let rules = get_rules(&input[0..divider]);

    let updates: Vec<Vec<i32>> = input[divider + 1..]
        .iter()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, updates)
}

fn score_updates(updates: Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .map(|update| update.iter().nth(update.len() / 2).unwrap())
        .sum::<i32>()
}

pub fn part1(input: &Vec<String>) -> i32 {
    let (rules, updates) = parse(input);

    let valid: Vec<Vec<i32>> = updates
        .iter()
        .filter(|update| {
            let mut encountered: HashMap<i32, i32> = HashMap::new();
            for page in update.iter() {
                match rules.get(page) {
                    Some(dependents) => {
                        for dependent in dependents {
                            if encountered.contains_key(dependent) {
                                return false;
                            }
                        }
                    }
                    _ => {}
                };

                encountered.insert(*page, *page);
            }
            encountered.len() == update.len()
        })
        .map(|update| update.clone())
        .collect();

    score_updates(valid)
}

pub fn part2(input: &Vec<String>) -> i32 {
    let (rules, updates) = parse(input);

    let invalid: Vec<Vec<i32>> = updates
        .iter()
        .filter(|update| {
            let mut encountered: HashMap<i32, i32> = HashMap::new();
            for page in update.iter() {
                match rules.get(page) {
                    Some(dependents) => {
                        for dependent in dependents {
                            if encountered.contains_key(dependent) {
                                return true;
                            }
                        }
                    }
                    _ => {}
                };

                encountered.insert(*page, *page);
            }
            false
        })
        .map(|update| update.clone())
        .collect();

    let reordered = invalid.iter().map(|p| reorder(p, &rules)).collect();

    score_updates(reordered)
}

fn reorder(pages: &Vec<i32>, proceeds: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    let mut follows = HashMap::new();

    proceeds.iter().for_each(|(key, value)| {
        value.iter().for_each(|v| {
            if follows.contains_key(v) {
                let entry: &mut Vec<i32> = follows.get_mut(v).unwrap();
                entry.push(*key);
            } else {
                follows.insert(*v, vec![*key]);
            }
        });
    });

    for page in pages.clone() {
        let mut index = 0;
        result.iter().enumerate().for_each(|(i, p)| {
            if follows.contains_key(&page) {
                    let antecedents = follows.get(&page).unwrap();
                    if antecedents.contains(&p) {
                        index = i + 1;
                    }
                }
        });

        result.insert(index, page);
    }

    result
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
        assert_eq!(result, 143);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 123);
    }
}

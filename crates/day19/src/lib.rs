use std::collections::HashSet;

fn parse(input: &Vec<String>) -> (HashSet<String>, Vec<String>) {
    let inventory = input.first().unwrap().split(", ").map(|s| s.to_string()).collect();
    let patterns = input[2..].iter().cloned().collect();
    println!("Inventory: {:?}", inventory);
    (inventory, patterns)
}

#[derive(Clone, Debug)]
struct Pattern {
    index: usize,
    parts: Vec<String>,
}

// impl Pattern {
//     fn get_pattern(&self) -> String {
//         self.parts.join("")
//     }
// }


fn is_part(inventory: &HashSet<String>, width: usize, pattern: &String) -> Option<String> {
    // println!("\tpattern: {pattern} width: {width}");

    if pattern.len() >= width && inventory.contains(&pattern[0..width]) {
        Some(pattern[0..width].to_string())
    } else {
        None
    }

    // match pattern.len() >= width {
    //     true => {
    //         let key = pattern[0..width].to_string();
    //         // println!("\t\tKey: {key}");
    //         match inventory.contains(&key) {
    //             true => Some(key),
    //             _ => None
    //         }
    //     }
    //     false => None
    // }
}


fn can_build(inventory: &HashSet<String>, patterns: &Vec<String>) -> Vec<String> {
    patterns.iter().filter(|p| {
        println!("{:?}: Examining {p}", std::time::Instant::now());
        let mut options: Vec<Pattern> = vec![];
        let key_size = inventory.iter().map(|k| k.len()).max().unwrap() + 1;

        // first iteration
        for i in 1..key_size {
            match is_part(inventory, i, p) {
                Some(x) => options.push(Pattern { index: i, parts: vec![x] }),
                _ => ()
            }
        }

        // pattern walking
        while !options.is_empty() && !options.iter().any(|_o| _o.index == p.len()) {
            let mut opt2 = vec![];

            for opt in options {
                for i in 1..key_size {
                    match is_part(inventory, i, &p[opt.index..].to_string()) {
                        Some(x) => {
                            let mut o = opt.clone();
                            println!("\tAdding {x} at {} to {}", o.index, o.index + 1);
                            o.index = o.index + i;
                            o.parts.push(x);
                            opt2.push(o);
                        }
                        _ => ()
                    }
                }
            }

            options = opt2;
        };

        !options.is_empty()
    }).cloned().collect()
}


pub fn part1(input: &Vec<String>) -> i64 {
    let (inventory, patterns) = parse(input);
    let buildable = can_build(&inventory, &patterns);
    println!("Buildable: {:?}", buildable);
    buildable.len() as i64
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
    fn part1_sample_valid() {
        let input = get_sample();
        let result = part1(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_sample_valid() {
        let input = get_sample();
        let result = part2(&input);
        assert_eq!(result, 0);
    }
}

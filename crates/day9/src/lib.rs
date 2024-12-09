#[derive(Clone, PartialEq, Eq)]
enum FileRef {
    File(i64, usize),
    Space,
}

impl FileRef {
    fn as_str(&self) -> &str {
        match self {
            FileRef::Space => ".",
            FileRef::File(x, _) => {
                let index = *x as usize % TOKENS.len();
                &TOKENS[index..index + 1]
            }
        }
    }
}

const TOKENS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
impl std::fmt::Debug for FileRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn read_map(input: &String) -> Vec<FileRef> {
    let mut id = 0;
    let mut files = Vec::new();

    for chunk in input.as_bytes().chunks(2) {
        let size = std::str::from_utf8(&chunk[0..1])
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let space = match &chunk.len() {
            2 => std::str::from_utf8(&chunk[1..2])
                .unwrap()
                .parse::<usize>()
                .unwrap_or(0),
            _ => 0,
        };
        for _ in 0..size {
            files.push(FileRef::File(id, size));
        }
        for _ in 0..space {
            files.push(FileRef::Space);
        }
        id += 1;
    }

    files
}

fn compress_blocks(disk: &Vec<FileRef>) -> Vec<FileRef> {
    let mut compressed = disk.clone();

    let mut write = 0;

    for read in (0..disk.len()).rev() {
        match disk[read] {
            FileRef::Space => (),
            FileRef::File(id, size) => {
                while compressed[write] != FileRef::Space && write < read {
                    write += 1;
                }
                if write >= read {
                    println!("midpoint detected: Write: {write}, Read: {read}");
                    break;
                }
                compressed[write] = FileRef::File(id, size);
                compressed[read] = FileRef::Space;
            }
        }
    }

    compressed
}

fn compress_first_fit(disk: &Vec<FileRef>) -> Vec<FileRef> {
    let mut compressed = disk.clone();

    let mut read = disk.len() - 1;
    while read > 0 {
        let mut write = 0;

        read -= match compressed[read] {
            FileRef::Space => 1,
            FileRef::File(id, size) => {
                while !compressed[write..write + size]
                    .iter()
                    .all(|x| *x == FileRef::Space)
                    && write < read
                    && write + size < disk.len()
                {
                    write += 1;
                }

                if compressed[write..write + size]
                    .iter()
                    .all(|x| *x == FileRef::Space) {
                    for i in 0..size {
                        compressed[write + i] = FileRef::File(id, size);
                        compressed[read - i] = FileRef::Space;
                    }
                    size
                } else {
                    1
                }
            }
        }
    }

    compressed
}

fn calculate_checksum(disk: &Vec<FileRef>) -> i64 {
    disk.iter().enumerate().fold(0, |acc, (i, &ref c)| match c {
        FileRef::Space => acc,
        FileRef::File(x, _) => acc + (x * i as i64),
    })
}

pub fn part1(input: &Vec<String>) -> i64 {
    let files = read_map(&input.first().unwrap());
    let compressed = compress_blocks(&files);
    calculate_checksum(&compressed)
}

pub fn part2(input: &Vec<String>) -> i64 {
    let files = read_map(&input.first().unwrap());
    let compressed = compress_first_fit(&files);
    calculate_checksum(&compressed)
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
    fn disk_layout_is_valid() {
        let input = get_sample();
        let files = read_map(&input.first().unwrap());

        assert_eq!(
            files.iter().map(|f| format!("{:?}", f)).collect::<String>(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn compress_blocks_is_valid() {
        let input = get_sample();
        let files = read_map(&input.first().unwrap());
        let compressed = compress_blocks(&files);
        assert_eq!(
            compressed
                .iter()
                .map(|f| format!("{:?}", f))
                .collect::<String>(),
            "0099811188827773336446555566.............."
        );
    }

    #[test]
    fn compress_first_fit_is_valid() {
        let input = get_sample();
        let files = read_map(&input.first().unwrap());
        let compressed = compress_first_fit(&files);
        assert_eq!(
            compressed
                .iter()
                .map(|f| format!("{:?}", f))
                .collect::<String>(),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }

    #[test]
    fn sample_part1() {
        let input = get_sample();
        assert_eq!(part1(&input), 1928);
    }

    #[test]
    fn sample_part2() {
        let input = get_sample();
        assert_eq!(part2(&input), 2858);
    }
}
